//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta787 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2599;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2600;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta787<F: Float>(t18662: F, t41070: F, t686: F, t72: F, t18658: F, t786: F, t789: F, t18796: F, t2465: F, t2470: F, t18811: F, t2435: F, t18825: F, t2453: F, t2458: F, t6042: F, t18785: F, t689: F, t779: F, t18316: F, t887: F, t2439: F, t2440: F, t6049: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61348, t61351, t61355, t61361) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2599::<F>(t18662, t41070, t686, t72, t18658, t786, t789, t18796, t2465, t2470, t18811, t2435);
        let (t61367, t61371, t61378, t61385, t61397) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2600::<F>(t18825, t2435, t2453, t2458, t6042, t18785, t689, t779, t18316, t887, t2439, t2440, t6049);
    (t61348, t61351, t61355, t61361, t61367, t61371, t61378, t61385, t61397)
}
