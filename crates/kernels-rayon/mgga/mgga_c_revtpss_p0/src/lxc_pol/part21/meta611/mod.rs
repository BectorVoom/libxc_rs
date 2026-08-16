//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2362;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta611(t2523: f64, t9318: f64, t2596: f64, t746: f64, t9385: f64, t760: f64, t186: f64, t2698: f64, t685: f64, t755: f64, t10558: f64, t177: f64, t762: f64, t2491: f64, t2495: f64, t39871: f64, t10326: f64, t706: f64, t750: f64, t9419: f64, t72: f64, t757: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40094, t40097, t40099, t40101, t40103, t40108) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2362(t2523, t9318, t2596, t746, t9385, t760, t186, t2698, t685, t755, t10558, t177, t762);
        let (t40113, t40115, t40119, t40121, t40125) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2363(t2491, t2495, t39871, t760, t10326, t706, t750, t2523, t9419, t10558, t72, t757);
    (t40094, t40097, t40099, t40101, t40103, t40108, t40113, t40115, t40119, t40121, t40125)
}
