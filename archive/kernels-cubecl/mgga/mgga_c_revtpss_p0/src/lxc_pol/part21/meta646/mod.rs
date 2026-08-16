//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta646<F: Float>(t11585: F, t945: F, t2935: F, t2967: F, t11509: F, t3006: F, t11501: F, t3014: F, t2866: F, t2873: F, t11298: F, t910: F) -> (F, F, F, F, F, F) {
        let (t41794, t41799, t41813, t41832, t41880, t41883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2431::<F>(t11585, t945, t2935, t2967, t11509, t3006, t11501, t3014, t2866, t2873, t11298, t910);
    (t41794, t41799, t41813, t41832, t41880, t41883)
}
