//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk578;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk579;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk580;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta114(t2619: f64, t755: f64, t72: f64, t752: f64, t757: f64, t2492: f64, t2596: f64, t745: f64, t760: f64, t123: f64, t192: f64, t676: f64, t762: f64, t2392: f64, t2400: f64, t2402: f64, t2416: f64, t2498: f64, t2518: f64, t2522: f64, t2525: f64, t2527: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2614: f64, t2617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2621, t2622, t2623, t2624, t2626) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk578(t2619, t755, t72, t752, t757, t2492, t2596, t745);
        let (t2628, t2629) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk579(t2626, t760, t123, t192);
        let t2630 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk580(t676, t762);
        let (t2632, t2633) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk581(t2629, t2630, t2392, t2400, t2402, t2416, t2498, t2518, t2522, t2525, t2527, t2562, t2569, t2579, t2587, t2610, t2614, t2617, t2621, t2624, t2628);
    (t2621, t2622, t2623, t2624, t2626, t2628, t2629, t2630, t2632, t2633)
}
