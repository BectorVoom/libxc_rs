//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta321(t13665: f64, t2630: f64, t1857: f64, t3860: f64, t3863: f64, t5566: f64, t749: f64, t512: f64, t9856: f64, t1468: f64, t9605: f64, t2: f64, t3874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13666, t13668, t13670, t13680, t13682, t13683, t13687, t13690) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1610(t13665, t2630, t1857, t3860, t3863, t5566, t749, t512, t9856, t1468, t9605, t2, t3874);
    (t13666, t13668, t13670, t13680, t13682, t13683, t13687, t13690)
}
