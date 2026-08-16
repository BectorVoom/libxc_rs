//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk642;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta133(t2988: f64, t3014: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t2935: f64, t2938: f64, t2943: f64, t2945: f64, t2963: f64, t2968: f64, t2971: f64, t2980: f64, t2982: f64, t2987: f64, t2989: f64, t3007: f64, t3012: f64, t311: f64, t946: f64, t955: f64, t965: f64, t974: f64, t300: f64, t960: f64) -> (f64, f64, f64, f64) {
        let (t3015, t3018) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk642(t2988, t3014, t2868, t2871, t2878, t2921, t2929, t2935, t2938, t2943, t2945, t2963, t2968, t2971, t2980, t2982, t2987, t2989, t3007, t3012, t311, t946, t955, t965, t974);
        let (t3019, t3021, t3022) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk643(t300, t3018, t2980, t960);
    (t3015, t3019, t3021, t3022)
}
