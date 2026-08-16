//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 767/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk767(t1509: f64, t898: f64, t41: f64, t1531: f64, t2463: f64, t2: f64, t2483: f64, t464: f64, t2333: f64, t2850: f64, t2271: f64, t2810: f64) -> (f64, f64, f64, f64, f64) {
    let t7030 = t898 * t1509;
    let t7031 = t41 * t7030;
    let t7032 = t2463 * t1531;
    let t7034 = t2483 * t2;
    let t7035 = t7034 * t464;
    let t7036 = 0.36622894612013090108e-3_f64 * t7035;
    let t7040 = t2850 * t2333;
    let t7048 = 0.4726e1_f64 * t2271 * t2810;
    (t7031, t7032, t7036, t7040, t7048)
}
