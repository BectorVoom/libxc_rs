//! Scaled exponential integral E₁ for CubeCL kernels.
//!
//! `xc_e1_scaled(x) = exp(x) * E₁(x)` where E₁ is the exponential integral.
//! Based on libxc's `expint_e1.c` which uses Chebyshev series (SLATEC/GSL origin).
//!
//! Matches the libxc C original (`xc_expint_e1_impl` in `expint_e1.c`) control
//! flow: uses `if/else` guards to evaluate only one Chebyshev series per call.
//! CubeCL 0.9.0 does not support `return`, so we use mutable result + `if/else`
//! instead of early returns. No runtime arrays; all coefficients are inlined.

use cubecl::prelude::*;

// Each Chebyshev evaluator computes: sum via Clenshaw recurrence b0 = 2x*b1 - b2 + c[i]
// then returns 0.5*(b0 - b2). Coefficients are hardcoded constants.

/// Chebyshev eval for AE11 series (39 coefficients), x in [-10, -4] region mapped to [-1,1].
#[cube]
fn cheb_ae11<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0: F = F::new(0.0);
    let mut b1: F = F::new(0.0);
    let mut b2: F = F::new(0.0);
    // Coefficients in reverse order (i = 38 down to 0)
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000000017_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000082_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000201_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000024_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000000716_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000001223_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000862_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000006074_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000005561_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000016383_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000045571_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000006246_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000200327_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000224338_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000607990_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000001692921_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000960151_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000008853444_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000003543928_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000040423282_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000047442060_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000179796603_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000343650105_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000868459898_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000002148771527_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000005118504888_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000012453235014_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000038711426349_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000058209273578_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000344809174450_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000056487164441_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000002804247688663_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000008113374735904_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000420236380882_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000093840434587471_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000649237843027216_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.004897651357459670_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.065088778513550150_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.121503239716065790_f64);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for AE12 series (25 coefficients), x in [-4, -1] region.
#[cube]
fn cheb_ae12<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0: F = F::new(0.0);
    let mut b1: F = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000058_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000244_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000716_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000537_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000010707_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000093709_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000492735_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000001769356_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000002905732_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000015830222_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000177476602_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000940724197_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000002844104870_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000662143777_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000066581901391_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000420650022012_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000001151381913647_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000002713395758640_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000041801320556301_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000143613366305483_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000435232492169391_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.005125843950185725_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.006764275590323141_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.158348850905782750_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.582417495134726740_f64);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for E11 series (19 coefficients), x in [-1, 0] region.
#[cube]
fn cheb_e11<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0: F = F::new(0.0);
    let mut b1: F = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000000000108_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000000002733_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000000065457_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000001479904_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000031481541_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000627627066_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000011673686816_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000201519974874_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000003209288853329_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000046816002303176_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000620286187580820_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000007388093356262168_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000078104901449841593_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000721107776966009185_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.05692503191092901938_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.37337293866277945612_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-1.95540581886314195070_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(7.79407277874268027690_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-16.11346165557149402600_f64);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for E12 series (16 coefficients), x in (0, 1] region.
#[cube]
fn cheb_e12<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0: F = F::new(0.0);
    let mut b1: F = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000000000315_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000000010148_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000000306291_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000008635897_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000226362142_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000005485141480_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000122076581374_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000002476417211390_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000045377325690753_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000742999951611943_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00010731029253063780_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00134617078051068022_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.01441912402469889073_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.13031820798497005440_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.04272398606220957700_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.03739021479220279500_f64);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for AE13 series (25 coefficients), x in [1, 4] region.
#[cube]
fn cheb_ae13<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0: F = F::new(0.0);
    let mut b1: F = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000000023_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000000094_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000000383_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000001568_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000006457_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000026804_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000000112211_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000000474132_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000002023672_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000008733026_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000038145706_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000000168864333_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000000758754209_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000003466802211_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000016143270567_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000000076823455870_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000000374943193568_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000001885368984916_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000009827812880247_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.000053564132129618_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.000309118337720603_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.001926845187381145_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.013432266247902779_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.112535243483660900_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.605773246640603460_f64);
    F::new(0.5) * (b0 - b2)
}

/// Chebyshev eval for AE14 series (26 coefficients), x > 4 region.
#[cube]
fn cheb_ae14<F: Float>(x: F) -> F {
    let twox = F::new(2.0) * x;
    let mut b0: F = F::new(0.0);
    let mut b1: F = F::new(0.0);
    let mut b2: F = F::new(0.0);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000000005_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000000016_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000000048_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000000148_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000000461_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000001463_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000004729_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000015592_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000052538_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000000181224_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000000641148_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000002331588_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000008737853_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000033846628_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000000135995766_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000000569232420_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000002495030440_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000011526808397_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00000056596491457_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00000298562751447_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00001717332998937_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00010999134432661_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.00080975594575573_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(0.00722410154374659_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.08648117855259871_f64);
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + F::cast_from(-0.18929180007530170_f64);
    F::new(0.5) * (b0 - b2)
}

/// Scaled exponential integral: exp(x) * E₁(x).
///
/// This is `xc_E1_scaled(x)` from libxc, equivalent to `xc_expint_e1_impl(x, 1)`.
/// Uses `if/else` guards to evaluate only the active Chebyshev region.
#[cube]
pub fn xc_e1_scaled<F: Float>(x: F) -> F {
    let mut e1: F = F::new(0.0);

    if x <= F::new(-10.0) {
        // Region 1
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae11::<F>(F::new(20.0) / x + F::new(1.0)));
    } else if x <= F::new(-4.0) {
        // Region 2
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae12::<F>((F::new(40.0) / x + F::new(7.0)) / F::new(3.0)));
    } else if x <= F::new(-1.0) {
        // Region 3
        e1 = F::exp(x) * (-F::ln(F::abs(x)) + cheb_e11::<F>((F::new(2.0) * x + F::new(5.0)) / F::new(3.0)));
    } else if x <= F::new(0.0) {
        // Region 4 (x == 0 is undefined; guard with select)
        let raw = F::exp(x) * (-F::ln(F::abs(x) + F::new(1.0e-300)) - F::new(0.6875) + x + cheb_e12::<F>(x));
        e1 = select(x == F::new(0.0), F::new(0.0), raw);
    } else if x <= F::new(1.0) {
        // Region 5
        e1 = F::exp(x) * (-F::ln(x + F::new(1.0e-300)) - F::new(0.6875) + x + cheb_e12::<F>(x));
    } else if x <= F::new(4.0) {
        // Region 6
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae13::<F>((F::new(8.0) / x - F::new(5.0)) / F::new(3.0)));
    } else {
        // Region 7: x > 4
        let s = F::new(1.0) / x;
        e1 = s * (F::new(1.0) + cheb_ae14::<F>(F::new(8.0) / x - F::new(1.0)));
    }

    e1
}
