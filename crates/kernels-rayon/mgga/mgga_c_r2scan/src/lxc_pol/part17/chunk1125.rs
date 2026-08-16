//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1125/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1125(t2333: f64, t2526: f64, t10655: f64, t11603: f64, t10922: f64, t11572: f64, t3308: f64, t3429: f64, t10810: f64, t3692: f64, t10935: f64, t2816: f64, t3446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40491 = t2333 * t2526;
    let t40513 = t10655 * t11603;
    let t40515 = t10922 * t11603;
    let t40518 = t3429 * t3308 * t11572;
    let t40556 = t3429 * t10810 * t3692;
    let t40559 = t3446 * t10935 * t2816;
    (t40491, t40513, t40515, t40518, t40556, t40559)
}
