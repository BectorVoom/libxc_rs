//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta77 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk466;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk467;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk468;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk469;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta77<F: Float>(t200: F, t202: F, t205: F, t262: F, t198: F, t206: F, t261: F, t125: F, t215: F, t123: F, t781: F, t124: F, t68: F, t138: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2375, t2382, t2393, t2403) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk466::<F>(t200, t202, t205, t262, t198, t206);
        let (t2410, t2411) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk467::<F>(t261);
        let t2434 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk468::<F>(t125, t215);
        let t2435 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk469::<F>(t123, t2434);
        let (t2437, t2438, t2439) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk470::<F>(t2435, t781, t124, t68, t138);
    (t2375, t2382, t2393, t2403, t2410, t2411, t2434, t2435, t2437, t2438, t2439)
}
