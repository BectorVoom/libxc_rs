//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta814 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2920;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta814(t2453: f64, t3908: f64, t4067: f64, t10115: f64, t1421: f64, t10168: f64, t3920: f64, t10174: f64, t9676: f64, t123: f64, t2434: f64, t3915: f64, t4131: f64, t10175: f64, t9686: f64, t1420: f64, t4075: f64, t786: f64, t2439: f64, t3895: f64, t4132: f64, t1359: f64, t39501: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47510, t47512, t47516, t47520, t47521, t47525) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2920(t2453, t3908, t4067, t10115, t1421, t10168, t3920, t10174, t9676, t123, t2434, t3915, t4131);
        let (t47527, t47530, t47534, t47561, t47567) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2921(t10175, t9686, t1420, t4075, t786, t2439, t3895, t4132, t1359, t39501, t10115, t555);
    (t47510, t47512, t47516, t47520, t47521, t47525, t47527, t47530, t47534, t47561, t47567)
}
