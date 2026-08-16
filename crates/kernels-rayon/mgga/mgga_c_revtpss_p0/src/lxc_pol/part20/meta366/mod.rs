//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1338;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta366(t2523: f64, t9323: f64, t9318: f64, t2596: f64, t746: f64, t9385: f64, t760: f64, t186: f64, t2698: f64, t685: f64, t755: f64, t10326: f64, t10599: f64, t4401: f64, t10558: f64, t177: f64, t762: f64, t150: f64, t190: f64, t39854: f64, t2491: f64, t2495: f64, t39871: f64, t10433: f64, t2398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40093, t40095, t40097, t40099, t40101, t40103, t40106) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1338(t2523, t9323, t9318, t2596, t746, t9385, t760, t186, t2698, t685, t755, t10326, t10599, t4401);
        let (t40109, t40111, t40113, t40115, t40117) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1339(t10558, t177, t762, t150, t190, t39854, t2491, t2495, t39871, t760, t10433, t2398);
    (t40093, t40095, t40097, t40099, t40101, t40103, t40106, t40109, t40111, t40113, t40115, t40117)
}
