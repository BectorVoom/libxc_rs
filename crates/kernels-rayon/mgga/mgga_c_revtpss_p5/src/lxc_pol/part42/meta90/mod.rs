//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta90(t2207: f64, t2209: f64, t572: f64, t573: f64, t10: f64, t17: f64, t576: f64, t580: f64, t15: f64, t22: f64, t11: f64, t14: f64) -> (f64, f64, f64, f64, f64) {
        let (t2212, t2219, t2221, t2223, t2224) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk526(t2207, t2209, t572, t573, t10, t17, t576, t580, t15, t22, t11, t14);
    (t2212, t2219, t2221, t2223, t2224)
}
