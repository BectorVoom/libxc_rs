//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1272/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1272(t1263: f64, t3362: f64, t12256: f64, t13099: f64, t1224: f64, t140: f64, t1260: f64, t12966: f64, t12987: f64, t15687: f64, t3623: f64, t3782: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17202 = t1263 * t3362;
    let t17235 = t13099 * t12256;
    let t17240 = t140 * t1224;
    let t17261 = t12966 * t1260;
    let t17344 = t12987 * t1260;
    let t17350 = t3623 * t15687;
    let t17351 = t3782 * t17350;
    (t17202, t17235, t17240, t17261, t17344, t17350, t17351)
}
