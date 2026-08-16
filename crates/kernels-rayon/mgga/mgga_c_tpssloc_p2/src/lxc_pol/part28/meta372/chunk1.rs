//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1416/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1416(t1603: f64, t3166: f64, t13939: f64, t381: f64, t1049: f64, t4552: f64, t1052: f64, t1066: f64, t13736: f64, t13743: f64, t14527: f64, t14529: f64, t14532: f64, t3026: f64, t3169: f64, t3207: f64, t388: f64, t4660: f64, t4665: f64, t4694: f64) -> f64 {
    let t14534 = t1603 * t3166;
    let t14536 = t13939 * t381;
    let t14538 = t4552 * t1049;
    let t14543 = -6.0_f64 * t1052 * t13736 + 4.0_f64 * t1052 * t13743 - 2.0_f64 * t1066 * t14529 + t14527 * t388 + t14532 * t388 + t14534 * t388 + t14536 * t388 + 2.0_f64 * t14538 * t388 + 4.0_f64 * t3026 * t4665 - 2.0_f64 * t3026 * t4694 - 2.0_f64 * t3169 * t4694 - t3207 * t4660;
    t14543
}
