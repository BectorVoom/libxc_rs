//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta937 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3081;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta937<F: Float>(t1145: F, t141: F, t81226: F, t24294: F, t698: F, t24288: F, t24291: F, t68262: F, t68277: F, t68312: F, t68332: F, t68334: F, t68336: F, t68368: F, t68370: F, t12254: F, t81160: F, t43764: F, t81212: F, t3417: F, t81182: F, t81198: F, t81202: F, t81190: F, t81194: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81423, t81425, t81427, t81429, t81437) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3081::<F>(t1145, t141, t81226, t24294, t698, t24288, t24291, t68262, t68277, t68312, t68332, t68334, t68336, t68368, t68370);
        let (t81439, t81442, t81445, t81448, t81451, t81454, t81457) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3082::<F>(t12254, t141, t81160, t43764, t81212, t3417, t81182, t1145, t81198, t81202, t81190, t81194);
    (t81423, t81425, t81427, t81429, t81437, t81439, t81442, t81445, t81448, t81451, t81454, t81457)
}
