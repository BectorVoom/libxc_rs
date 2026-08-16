//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1190/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1190(t11199: f64, t11550: f64, t3262: f64, t3275: f64, t3472: f64, t39339: f64, t12210: f64, t37513: f64, t12197: f64, t498: f64, t3264: f64, t3465: f64, t40374: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41196 = 3.0_f64 / 2.0_f64 * t3262 * t11199 * t11550;
    let t41199 = 5.0_f64 / 8.0_f64 * t3275 * t3472 * t39339;
    let t41201 = 3.0_f64 / 4.0_f64 * t37513 * t12210;
    let t41202 = t498 * t12197;
    let t41205 = 3.0_f64 / 2.0_f64 * t3262 * t41202 * t3264;
    let t41208 = 3.0_f64 / 4.0_f64 * t3262 * t3465 * t40374;
    (t41196, t41199, t41201, t41202, t41205, t41208)
}
