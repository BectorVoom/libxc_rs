//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta318(t760: f64, t9387: f64, t2496: f64, t2523: f64, t9372: f64, t37: f64, t716: f64, t2626: f64, t9425: f64, t2609: f64, t606: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10596, t10597, t10604, t10605, t10608, t10611, t10612, t10613) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1761(t760, t9387, t2496, t2523, t9372, t37, t716, t2626, t9425, t2609, t606, t706);
    (t10596, t10597, t10604, t10605, t10608, t10611, t10612, t10613)
}
