//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1490;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta413(t2349: f64, t43: f64, t10227: f64, t96: f64, t100: f64, t613: f64, t10199: f64, t2175: f64, t2289: f64, t8264: f64, t31051: f64, t625: f64, t31027: f64, t31044: f64, t2184: f64, t4168: f64, t31127: f64, t571: f64, t2192: f64, t4153: f64, t1455: f64, t8302: f64, t116: f64, t31066: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t116942, t116946, t116957, t116968, t116969, t116971) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1490(t2349, t43, t10227, t96, t100, t613, t10199, t2175, t2289, t8264, t31051, t625);
        let (t116995, t117090, t117095, t117097, t117099, t117103) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1491(t31027, t31044, t2184, t4168, t31127, t571, t2192, t4153, t1455, t8302, t116, t31066);
    (t116942, t116946, t116957, t116968, t116969, t116971, t116995, t117090, t117095, t117097, t117099, t117103)
}
