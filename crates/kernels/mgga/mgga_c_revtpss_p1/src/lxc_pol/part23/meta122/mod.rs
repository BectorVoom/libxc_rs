//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk792;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk793;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk794;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk795;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk796;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk797;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk798;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk799;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk800;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta122<F: Float>(t1071: F, t359: F, t3140: F, t3143: F, t342: F, t335: F, t368: F, t3153: F, t3154: F, t1035: F, t357: F, t389: F, t1941: F, t268: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3291 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk792::<F>(t1071, t359);
        let t3298 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk793::<F>(t3140, t3143);
        let t3299 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk794::<F>(t3298, t342);
        let t3302 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk795::<F>(t335, t368);
        let (t3303, t3304) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk796::<F>(t3153, t3302, t3154);
        let t3316 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk797::<F>(t1035, t3140);
        let t3317 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk798::<F>(t3316, t342);
        let t3318 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk799::<F>(t3303, t357);
        let (t3335, t3336) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk800::<F>(t389);
        let t3356 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk801::<F>(t1941, t268, t404);
    (t3291, t3298, t3299, t3302, t3303, t3304, t3316, t3317, t3318, t3335, t3336, t3356)
}
