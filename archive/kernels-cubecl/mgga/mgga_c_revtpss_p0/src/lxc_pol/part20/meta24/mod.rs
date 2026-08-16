//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta24 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk188;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk189;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk190;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk191;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk192;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk193;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk194;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta24<F: Float>(t225: F, t487: F, t473: F, t460: F, t198: F, t336: F, t424: F, t452: F, t454: F, t265: F, t33: F, t57: F, t398: F, dens_threshold: F, rho1: F, zeta_threshold: F, t117: F, t93: F, t19: F, t22: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t488, t489) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk188::<F>(t225, t487, t473);
        let t490 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk189::<F>(t487, t489);
        let (t493, t494) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk190::<F>(t460, t490);
        let t495 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk191::<F>(t488, t494);
        let (t498, t504, t502) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk192::<F>(t460, t495, t198, t336, t424, t452, t454, t265);
        let t508 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk193::<F>(t33, t265, t504, t57, t398, dens_threshold, rho1, zeta_threshold);
        let t511 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk194::<F>(t117, t93);
        let t512 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk195::<F>(t19, t22);
    (t488, t489, t490, t493, t494, t495, t498, t504, t502, t508, t511, t512)
}
