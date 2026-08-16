//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk686;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk687;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta148(t3153: f64, t3302: f64, t3154: f64, t3300: f64, t1043: f64, t1071: f64, t1089: f64, t3133: f64, t378: f64, t1035: f64, t3140: f64, t342: f64, t357: f64, t3259: f64, t380: f64, t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t3043: f64, t3204: f64, t3223: f64, t3278: f64, t3283: f64, t3287: f64, t3288: f64, t3292: f64, t3295: f64, t3299: f64, t381: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3303, t3304, t3305, t3309, t3313, t3316) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk686(t3153, t3302, t3154, t3300, t1043, t1071, t1089, t3133, t378, t1035, t3140);
        let t3317 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk687(t3316, t342);
        let (t3318, t3319, t3322, t3325) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk688(t3303, t357, t3300, t3259, t380, t1024, t1083, t1087, t1090, t1093, t3043, t3204, t3223, t3278, t3283, t3287, t3288, t3292, t3295, t3299, t3305, t3309, t3313, t3317, t342, t381, t989);
    (t3303, t3304, t3305, t3309, t3313, t3316, t3317, t3318, t3319, t3322, t3325)
}
