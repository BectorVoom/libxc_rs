//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta30 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk207;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk208;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta30(t572: f64, t573: f64, t10: f64, t2: f64, t17: f64, t16: f64, t3: f64, t15: f64, t14: f64, t11: f64, t22: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t575, t576, t578, t579, t580) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk207(t572, t573, t10, t2, t17, t16, t3);
        let (t582, t583) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk208(t15, t580, t14, t2);
        let (t584, t586, t587, t588) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk209(t11, t583, t22, t21, t3);
    (t575, t576, t578, t579, t580, t582, t583, t584, t586, t587, t588)
}
