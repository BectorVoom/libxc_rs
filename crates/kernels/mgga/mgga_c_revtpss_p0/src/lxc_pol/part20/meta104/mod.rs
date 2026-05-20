//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk594;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk595;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk596;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta104<F: Float>(t290: F, t2875: F, t2924: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t941: F, t945: F, t307: F, t944: F, t302: F, t953: F, t954: F, t2904: F, t2882: F, t2890: F, t2898: F, t2900: F, t2906: F, t2910: F, t2913: F, t2916: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2925, t2926) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk594::<F>(t290);
        let (t2927, t2929, t2935, t2938, t2941) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk595::<F>(t2875, t2926, t2924, t2846, t2848, t2855, t2860, t2864, t941, t945, t307, t944);
        let (t2942, t2943, t2944) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk596::<F>(t2941, t302, t953);
        let (t2945, t2962) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk597::<F>(t2944, t954, t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
    (t2925, t2926, t2927, t2929, t2935, t2938, t2941, t2942, t2943, t2944, t2945, t2962)
}
