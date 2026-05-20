//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta376<F: Float>(t24236: F, t5312: F, t13046: F, t24544: F, t1042: F, t13053: F, t1803: F, t6601: F, t1222: F, t1235: F, t1261: F, t12853: F, t13042: F, t13052: F, t1797: F, t21053: F, t21088: F, t21091: F, t21102: F, t24636: F, t24640: F, t24644: F, t24649: F, t24652: F, t3711: F, t484: F) -> (F, F, F, F, F, F, F) {
        let (t24655, t24663, t24664, t24667, t24668, t24671, t24674) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1269::<F>(t24236, t5312, t13046, t24544, t1042, t13053, t1803, t6601, t1222, t1235, t1261, t12853, t13042, t13052, t1797, t21053, t21088, t21091, t21102, t24636, t24640, t24644, t24649, t24652, t3711, t484);
    (t24655, t24663, t24664, t24667, t24668, t24671, t24674)
}
