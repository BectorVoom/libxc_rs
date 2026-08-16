//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 799/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk799(t464: f64, t7034: f64, t2266: f64, t6599: f64, t910: f64, t2333: f64, t2850: f64) -> (f64, f64, f64) {
    let t7035 = t7034 * t464;
    let t7036 = 0.36622894612013090108e-3_f64 * t7035;
    let t7038 = t2266 * t6599 * t910;
    let t7039 = 3.0_f64 * t7038;
    let t7040 = t2850 * t2333;
    (t7036, t7039, t7040)
}
