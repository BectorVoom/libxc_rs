//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta899 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2859;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta899(t1469: f64, t4401: f64, t61266: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t50874: f64, t50884: f64, t77020: f64, t77021: f64, t77023: f64, t77024: f64, t77025: f64, t77026: f64, t77027: f64, t77028: f64, t77029: f64, t18305: f64, t4186: f64, t18576: f64, t62291: f64, t62302: f64, t50892: f64, t50893: f64, t189: f64, t22671: f64, t606: f64, t177: f64, t23211: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77032, t77033) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2859(t1469, t4401, t61266, t40067, t40072, t40167, t40171, t40184, t50874, t50884, t77020, t77021, t77023, t77024, t77025, t77026, t77027, t77028, t77029);
        let (t77036, t77038, t77039, t77040, t77041, t77045, t77047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2860(t18305, t4186, t4401, t18576, t62291, t62302, t50892, t50893, t189, t22671, t606, t177, t23211, t762);
    (t77032, t77033, t77036, t77038, t77039, t77040, t77041, t77045, t77047)
}
