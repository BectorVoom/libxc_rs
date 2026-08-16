//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk714;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta157(t1169: f64, t3471: f64, t1159: f64, t426: f64, t434: f64, t3453: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t448: f64, t1175: f64, t1179: f64, t1178: f64, t444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3472, t3475, t3476, t3477, t3478, t3479, t3480, t3483, t3488, t3489) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk714(t1169, t3471, t1159, t426, t434, t3453, t3356, t3358, t3365, t3370, t3374, t448);
        let (t3491, t3495) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk715(t1175, t1179, t1178, t444);
    (t3472, t3475, t3476, t3477, t3478, t3479, t3480, t3483, t3488, t3489, t3491, t3495)
}
