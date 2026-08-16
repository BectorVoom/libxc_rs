//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta277(t10199: f64, t112: f64, t2289: f64, t666: f64, t654: f64, t98: f64, t99: f64, t106: f64, t107: f64, t10: f64, t580: f64, t22: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10201, t10202, t10207, t10208, t10227, t10241, t10270, t10272) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1029(t10199, t112, t2289, t666, t654, t98, t99, t106, t107, t10, t580, t22, t576);
    (t10201, t10202, t10207, t10208, t10227, t10241, t10270, t10272)
}
