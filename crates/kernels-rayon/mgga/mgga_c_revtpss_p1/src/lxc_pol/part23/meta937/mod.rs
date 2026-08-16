//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta937 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3081;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta937(t1145: f64, t141: f64, t81226: f64, t24294: f64, t698: f64, t24288: f64, t24291: f64, t68262: f64, t68277: f64, t68312: f64, t68332: f64, t68334: f64, t68336: f64, t68368: f64, t68370: f64, t12254: f64, t81160: f64, t43764: f64, t81212: f64, t3417: f64, t81182: f64, t81198: f64, t81202: f64, t81190: f64, t81194: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81423, t81425, t81427, t81429, t81437) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3081(t1145, t141, t81226, t24294, t698, t24288, t24291, t68262, t68277, t68312, t68332, t68334, t68336, t68368, t68370);
        let (t81439, t81442, t81445, t81448, t81451, t81454, t81457) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3082(t12254, t141, t81160, t43764, t81212, t3417, t81182, t1145, t81198, t81202, t81190, t81194);
    (t81423, t81425, t81427, t81429, t81437, t81439, t81442, t81445, t81448, t81451, t81454, t81457)
}
