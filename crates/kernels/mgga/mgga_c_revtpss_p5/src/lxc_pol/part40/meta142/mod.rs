//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk672;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk673;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta142<F: Float>(t1046: F, t3172: F, t1041: F, t1066: F, t2862: F, t247: F, t283: F, t905: F, t66: F, t2853: F, t1047: F, t1063: F, t1068: F, t3082: F, t3083: F, t3086: F, t3091: F, t3097: F, t3101: F, t3106: F, t3112: F, t3115: F, t3120: F, t3124: F, t3127: F, t3130: F, t3136: F, t3150: F, t3157: F, t3161: F, t3164: F, t3169: F, t348: F, t1020: F, t1062: F) -> (F, F, F, F, F, F, F, F) {
        let (t3173, t3174, t3177, t3181) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk672::<F>(t1046, t3172, t1041, t1066, t2862, t247, t283, t905);
        let (t3182, t3184, t3187) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk673::<F>(t3181, t66, t2853, t247, t1041, t1047, t1063, t1068, t3082, t3083, t3086, t3091, t3097, t3101, t3106, t3112, t3115, t3120, t3124, t3127, t3130, t3136, t3150, t3157, t3161, t3164, t3169, t3174, t3177, t348);
        let t3188 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk674::<F>(t1020, t1062);
    (t3173, t3174, t3177, t3181, t3182, t3184, t3187, t3188)
}
