//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1030/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1030(t17307: f64, t480: f64, t1804: f64, t3655: f64, t1786: f64, t1260: f64, t12987: f64, t15687: f64, t3623: f64, t3782: f64, t1263: f64, t1794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17308 = t17307 * t480;
    let t17340 = t1804 * t3655;
    let t17342 = t1786 * t3655;
    let t17344 = t12987 * t1260;
    let t17350 = t3623 * t15687;
    let t17351 = t3782 * t17350;
    let t17352 = t1263 * t1794;
    (t17308, t17340, t17342, t17344, t17350, t17351, t17352)
}
