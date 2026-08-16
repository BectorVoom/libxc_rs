//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk682;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk683;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta146(t225: f64, t3259: f64, t385: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t1096: f64, t1086: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3261, t3264, t3268, t3269) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk682(t225, t3259, t385, t1071, t342, t1077, t384);
        let (t3270, t3271, t3278) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk683(t1096, t3269, t1086, t989);
    (t3261, t3264, t3268, t3269, t3270, t3271, t3278)
}
