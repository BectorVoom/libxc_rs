//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1440;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta466(t18562: f64, t2496: f64, t5825: f64, t749: f64, t2439: f64, t6041: f64, t780: f64, t785: f64, t18821: f64, t2471: f64, t18814: f64, t2435: f64, t18796: f64, t2465: f64, t2470: f64, t18811: f64, t18825: f64, t2453: f64, t2458: f64, t6042: f64, t2440: f64, t6049: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61296, t61303, t61324, t61330, t61337) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1440(t18562, t2496, t5825, t749, t2439, t6041, t780, t785, t18821, t2471, t18814, t2435);
        let (t61355, t61361, t61367, t61371, t61397) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1441(t18796, t2465, t2470, t18811, t2435, t18825, t2453, t2458, t6042, t2439, t2440, t6049);
    (t61296, t61303, t61324, t61330, t61337, t61355, t61361, t61367, t61371, t61397)
}
