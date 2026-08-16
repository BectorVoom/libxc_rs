//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta787 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2599;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2600;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta787(t18662: f64, t41070: f64, t686: f64, t72: f64, t18658: f64, t786: f64, t789: f64, t18796: f64, t2465: f64, t2470: f64, t18811: f64, t2435: f64, t18825: f64, t2453: f64, t2458: f64, t6042: f64, t18785: f64, t689: f64, t779: f64, t18316: f64, t887: f64, t2439: f64, t2440: f64, t6049: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61348, t61351, t61355, t61361) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2599(t18662, t41070, t686, t72, t18658, t786, t789, t18796, t2465, t2470, t18811, t2435);
        let (t61367, t61371, t61378, t61385, t61397) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2600(t18825, t2435, t2453, t2458, t6042, t18785, t689, t779, t18316, t887, t2439, t2440, t6049);
    (t61348, t61351, t61355, t61361, t61367, t61371, t61378, t61385, t61397)
}
