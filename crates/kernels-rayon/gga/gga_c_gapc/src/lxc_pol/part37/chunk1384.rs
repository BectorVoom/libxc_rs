//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1384/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1384(t34056: f64, t34060: f64, t34062: f64, t34066: f64, t34069: f64, t34071: f64, t34075: f64, t34079: f64, t34084: f64, t34088: f64, t34095: f64, t34098: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36807 = 0.14068374825384584215e-7_f64 * t34056;
    let t36808 = 0.46573198186092908864e-9_f64 * t34060;
    let t36809 = 0.49520679385353736436e-5_f64 * t34062;
    let t36812 = 0.67528199161846004232e-6_f64 * t34066;
    let t36813 = 0.40021712703254065176e-7_f64 * t34069;
    let t36814 = 0.40094868252346065012e-6_f64 * t34071;
    let t36815 = 0.26194149710963390811e-8_f64 * t34075;
    let t36816 = 0.32227054270378187512e-8_f64 * t34079;
    let t36817 = 0.60722656250000000004e-3_f64 * t34084;
    let t36818 = 0.88394205998751600035e-8_f64 * t34088;
    let t36820 = 0.67528199161846004232e-6_f64 * t34095;
    let t36821 = 0.78582449132890172432e-8_f64 * t34098;
    (t36807, t36808, t36809, t36812, t36813, t36814, t36815, t36816, t36817, t36818, t36820, t36821)
}
