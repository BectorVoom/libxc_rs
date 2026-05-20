//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk853;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk854;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk855;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta157<F: Float>(t162: F, t2611: F, t227: F, t73: F, t225: F, t2718: F, t213: F, t2783: F, t198: F, t205: F, t3014: F, t972: F, t3093: F, t357: F, t1065: F, t2857: F, t2852: F, t3181: F, t1062: F, t3204: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4401, t4415, t4503) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk853::<F>(t162, t2611, t227, t73, t225, t2718);
        let (t4504, t4514, t4541) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk854::<F>(t213, t4503, t2783, t198, t205);
        let (t4733, t4786, t4801) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk855::<F>(t3014, t972, t3093, t357, t1065, t2857);
        let (t4806, t4837) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk856::<F>(t2852, t3181, t1062, t3204);
    (t4401, t4415, t4503, t4504, t4514, t4541, t4733, t4786, t4801, t4806, t4837)
}
