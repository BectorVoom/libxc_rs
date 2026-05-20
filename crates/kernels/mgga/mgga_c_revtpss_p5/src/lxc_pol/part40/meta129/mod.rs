//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk634;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta129<F: Float>(t290: F, t2875: F, t2924: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t941: F, t945: F, t307: F, t944: F, t302: F, t953: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2941) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk634::<F>(t290, t2875, t2924, t2846, t2848, t2855, t2860, t2864, t941, t945, t307, t944);
        let (t2942, t2943, t2944) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk635::<F>(t2941, t302, t953);
    (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2942, t2943, t2944)
}
