//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1844/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1844(t12810: f64, t5352: f64, t3720: f64, t12269: f64, t247: f64, t3618: f64, t12277: f64, t1264: f64, t12273: f64, t1284: f64, t3555: f64, t3624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12811 = t12810 * t5352;
    let t12812 = t3720 * t12811;
    let t12816 = t247 * t3618 * t12269;
    let t12822 = t247 * t1264 * t12277;
    let t12828 = t247 * t1264 * t12273;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    (t12811, t12812, t12816, t12822, t12828, t12831, t12832)
}
