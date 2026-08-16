//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta400(t40097: f64, t760: f64, t186: f64, t2698: f64, t685: f64, t755: f64, t2491: f64, t2495: f64, t39871: f64, t2598: f64, t9321: f64, t39875: f64, t9367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40099, t40101, t40103, t40113, t40115, t40129, t40131, t40135) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1335(t40097, t760, t186, t2698, t685, t755, t2491, t2495, t39871, t2598, t9321, t39875, t9367);
    (t40099, t40101, t40103, t40113, t40115, t40129, t40131, t40135)
}
