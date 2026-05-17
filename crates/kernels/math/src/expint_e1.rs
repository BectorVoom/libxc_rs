//! Scaled exponential integral E₁ for CubeCL kernels.
//!
//! `xc_e1_scaled(x) = exp(x) * E₁(x)` where E₁ is the exponential integral.
//! Based on libxc's `expint_e1.c` which uses Chebyshev series (SLATEC/GSL origin).
//! Generic over `<F: Float>` to support both f64 and f32.
//!
//! Matches the libxc C original (`xc_expint_e1_impl` in `expint_e1.c`) control
//! flow: uses `if/else` guards to evaluate only one Chebyshev series per call.
//! CubeCL 0.10 does not support `return`, so we use mutable result + `if/else`
//! instead of early returns. No runtime arrays; all coefficients are inlined.

use cubecl::prelude::*;

// Each Chebyshev evaluator computes: sum via Clenshaw recurrence b0 = 2x*b1 - b2 + c[i]
// then returns 0.5*(b0 - b2). Coefficients are hardcoded constants.

/// Chebyshev eval for AE11 series (39 coefficients), x in [-10, -4] region mapped to [-1,1].
#[cube]
fn cheb_ae11<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0 = F::new(0.0);
    let mut b1 = F::new(0.0);
    let mut b2: F = F::new(0.0);
    // Coefficients in reverse order (i = 38 down to 0)
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000000017);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000000082);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000000201);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000002);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000000071);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000001223);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000000862);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000607);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000005561);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000016383);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000045571);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.0000000000000062);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000200327);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000224338);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000607990);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000001692921);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000960151);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000008853);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000003543928);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000040423282);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000047442060);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000179796603);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000343650105);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000868459898);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000002148771527);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000005118504888);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000001245323501);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000038711426349);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000058209273578);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000344809174450);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000056487164441);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000002804247688663);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000811337473590);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000420236380882);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000093840434587471);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00064923784302721);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.004897651357459670);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.065088778513550150);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.121503239716065790);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for AE12 series (25 coefficients), x in [-4, -1] region.
#[cube]
fn cheb_ae12<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0 = F::new(0.0);
    let mut b1 = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000000058);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.0000000000000002);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000071);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000000537);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000010707);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000093709);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000492735);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000176935);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000002905732);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000015830222);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000177476602);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000940724197);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000002844104870);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000662143777);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000066581901391);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000420650022012);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000001151381913647);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000002713395758640);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000041801320556301);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000143613366305483);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000435232492169391);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.005125843950185725);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.006764275590323141);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.158348850905782750);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.582417495134726740);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for F::new(E11) series (19 coefficients), x in [-1, 0] region.
#[cube]
fn cheb_e11<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0 = F::new(0.0);
    let mut b1 = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000000108);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000000002733);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000065457);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.0000000000000147990);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000031481541);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000006276270);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.0000000001167368681);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.0000000020151997487);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000003209288853329);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.0000000004681600230317);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000620286187580820);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000007388093356262168);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000078104901449841593);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000721107776966009185);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.05692503191092901938);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.37337293866277945612);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(1.95540581886314195070);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(7.79407277874268027690);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(16.11346165557149402600);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for F::new(E12) series (16 coefficients), x in (0, 1] region.
#[cube]
fn cheb_e12<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0 = F::new(0.0);
    let mut b1 = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000000000315);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000010148);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000000306291);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000008635897);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000226362142);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000005485141480);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.0000000012207658137);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000002476417211390);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000045377325690753);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000742999951611943);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00010731029253063780);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00134617078051068022);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.01441912402469889073);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.13031820798497005440);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.04272398606220957700);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.03739021479220279500);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for F::new(AE13) series (25 coefficients), x in [1, 4] region.
#[cube]
fn cheb_ae13<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0 = F::new(0.0);
    let mut b1 = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000000023);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000009);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000000383);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000001568);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000006457);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000002680);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000000112211);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000000474132);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000002023672);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000873302);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000003814570);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000000168864333);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000000758754209);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000003466802211);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000016143270567);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000076823455870);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000000374943193568);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000188536898491);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000009827812880247);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000053564132129618);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.000309118337720603);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.001926845187381145);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.013432266247902779);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.112535243483660900);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.605773246640603460);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for F::new(AE14) series (26 coefficients), x > 4 region.
#[cube]
fn cheb_ae14<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0 = F::new(0.0);
    let mut b1 = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000005);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.0000000000000001);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000048);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000000148);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000000461);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000001463);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000004729);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000000015592);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000052538);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.0000000000018122);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000000641148);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000002331588);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000000008737853);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000033846628);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.000000001359957);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000000569232420);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000002495030440);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000011526808397);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00000056596491457);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00000298562751447);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00001717332998937);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00010999134432661);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.00080975594575573);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::new(0.00722410154374659);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.08648117855259871);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -F::new(0.18929180007530170);
    F::new(0.5) * (b0 - b2)
}

/// Scaled exponential integral: exp(x) * E₁(x).
///
/// Scaled exponential integral E₁: xc_e1_scaled(x) = exp(x) * E₁(x).
/// Uses `if/else` guards to evaluate only the active Chebyshev region.
#[cube]
pub fn xc_e1_scaled<F: Float>(x: F) -> F {
    let mut e1 = F::new(0.0);

    if x <= -F::new(10.0) {
        // Region 1
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae11(F::new(20.0) / x + F::new(1.0)));
    } else if x <= -F::new(4.0) {
        // Region 2
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae12((F::new(40.0) / x + F::new(7.0)) / F::new(3.0)));
    } else if x <= -F::new(1.0) {
        // Region 3
        e1 = F::exp(x) * (-F::ln(F::abs(x)) + cheb_e11((F::new(2.0) * x + F::new(5.0)) / F::new(3.0)));
    } else if x <= F::new(0.0) {
        // Region 4 (x == 0 is undefined; guard with select)
        let raw = F::exp(x) * (-F::ln(F::abs(x) + F::new(1.0e-300)) - F::new(0.6875) + x + cheb_e12(x));
        e1 = select(x == F::new(0.0), F::new(0.0), raw);
    } else if x <= F::new(1.0) {
        // Region 5
        e1 = F::exp(x) * (-F::ln(x + F::new(1.0e-300)) - F::new(0.6875) + x + cheb_e12(x));
    } else if x <= F::new(4.0) {
        // Region 6
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae13((F::new(8.0) / x - F::new(5.0)) / F::new(3.0)));
    } else {
        // Region 7: x > 4
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae14(F::new(8.0) / x - F::new(1.0)));
    }

    e1
}
