//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk722;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk723;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk724;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta160(t1198: f64, t3531: f64, t1188: f64, t3495: f64, t3497: f64, t1196: f64, t1179: f64, t3515: f64, t3520: f64, t3523: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t459: f64, t1203: f64, t1208: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3533, t3535, t3537, t3539, t3541, t3543, t3545, t3546, t3551) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk722(t1198, t3531, t1188, t3495, t3497, t1196, t1179, t3515, t3520, t3523, t3356, t3358, t3365, t3370, t3374);
        let t3552 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk723(t3551, t459);
        let t3555 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk724(t1203, t1208);
        let t3556 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk725(t3555, t487);
    (t3533, t3535, t3537, t3539, t3541, t3543, t3545, t3546, t3551, t3552, t3555, t3556)
}
