//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk749;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk750;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta146(t1046: f64, t3172: f64, t1041: f64, t1066: f64, t2862: f64, t247: f64, t283: f64, t905: f64, t66: f64, t2853: f64, t1047: f64, t1063: f64, t1068: f64, t3082: f64, t3083: f64, t3086: f64, t3091: f64, t3097: f64, t3101: f64, t3106: f64, t3112: f64, t3115: f64, t3120: f64, t3124: f64, t3127: f64, t3130: f64, t3136: f64, t3150: f64, t3157: f64, t3161: f64, t3164: f64, t3169: f64, t348: f64, t1020: f64, t1062: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3173, t3174, t3177, t3181) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk749(t1046, t3172, t1041, t1066, t2862, t247, t283, t905);
        let (t3182, t3184, t3187) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk750(t3181, t66, t2853, t247, t1041, t1047, t1063, t1068, t3082, t3083, t3086, t3091, t3097, t3101, t3106, t3112, t3115, t3120, t3124, t3127, t3130, t3136, t3150, t3157, t3161, t3164, t3169, t3174, t3177, t348);
        let t3188 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk751(t1020, t1062);
    (t3173, t3174, t3177, t3181, t3182, t3184, t3187, t3188)
}
