//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2302/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2302(t1264: f64, t24240: f64, t247: f64, t1794: f64, t3603: f64, t20800: f64, t3720: f64, t471: f64, t6573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24726 = t247 * t1264 * t24240;
    let t24729 = t3603 * t1794;
    let t24730 = t20800 * t24729;
    let t24731 = t3720 * t24730;
    let t24734 = t1794 * t471;
    let t24735 = t20800 * t24734;
    let t24736 = t3720 * t24735;
    let t24739 = t6573 * t1794;
    (t24726, t24729, t24730, t24731, t24734, t24735, t24736, t24739)
}
