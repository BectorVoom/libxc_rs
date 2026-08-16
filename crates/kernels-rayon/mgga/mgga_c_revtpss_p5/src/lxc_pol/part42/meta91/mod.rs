//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta91(t22: f64, t2224: f64, t584: f64, t588: f64, t20: f64, t27: f64, t12: f64, t19: f64, t592: f64, t596: f64, t21: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2226, t2228, t2230, t2231, t2233, t2235, t2236, t2237, t2239) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk527(t22, t2224, t584, t588, t20, t27, t12, t19, t592, t596, t21, t25);
    (t2226, t2228, t2230, t2231, t2233, t2235, t2236, t2237, t2239)
}
