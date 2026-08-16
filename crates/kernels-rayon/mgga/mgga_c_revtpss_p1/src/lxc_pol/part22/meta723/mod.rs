//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta723(t2523: f64, t9318: f64, t2596: f64, t746: f64, t9385: f64, t760: f64, t186: f64, t2698: f64, t685: f64, t755: f64, t2491: f64, t2495: f64, t39871: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t40094, t40097, t40099, t40101, t40103, t40113) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2779(t2523, t9318, t2596, t746, t9385, t760, t186, t2698, t685, t755, t2491, t2495, t39871);
    (t40094, t40097, t40099, t40101, t40103, t40113)
}
