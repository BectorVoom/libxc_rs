//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 474/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk474(t1457: f64, t503: f64, t475: f64, t1486: f64, t469: f64, t382: f64, t41: f64, t3783: f64, t484: f64, t492: f64, t497: f64, t1414: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4169 = 1.0_f64 / t1457 / t503;
    let t4170 = t475 * t4169;
    let t4203 = t1486 * t469;
    let t4204 = t41 * t382;
    let t4208 = t484 * t3783;
    let t4209 = t4208 * sigma0;
    let t4229 = t492 * t497;
    let t4230 = t1414 * t4229;
    (t4169, t4170, t4203, t4204, t4208, t4209, t4229, t4230)
}
