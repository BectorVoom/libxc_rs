//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta30 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk193;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk194;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta30(t15: f64, t580: f64, t14: f64, t2: f64, t11: f64, t22: f64, t21: f64, t3: f64, t20: f64, t12: f64, t19: f64, t27: f64, t579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t582, t583) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk193(t15, t580, t14, t2);
        let (t584, t586, t587, t588) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk194(t11, t583, t22, t21, t3);
        let (t590, t592, t594, t595, t596) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk195(t20, t588, t12, t19, t2, t27, t21, t579);
    (t582, t583, t584, t586, t587, t588, t590, t592, t594, t595, t596)
}
