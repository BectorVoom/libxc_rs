//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta123 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk791;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk792;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk793;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk794;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta123<F: Float>(t290: F, t2875: F, t2924: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t941: F, t945: F, t307: F, t944: F, t302: F, t953: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2925, t2926) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk791::<F>(t290);
        let (t2927, t2929, t2930, t2935, t2938) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk792::<F>(t2875, t2926, t2924, t2846, t2848, t2855, t2860, t2864, t941, t945);
        let (t2941, t2942) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk793::<F>(t307, t944);
        let t2943 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk794::<F>(t2942, t302);
        let t2944 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk795::<F>(t953);
    (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2941, t2942, t2943, t2944)
}
