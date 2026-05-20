//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta270<F: Float>(t4003: F, t9768: F, t9934: F, t2661: F, t532: F, t549: F, t240: F, t72: F, t828: F, t9400: F, t595: F, t66: F) -> (F, F, F, F, F, F, F, F) {
        let (t9935, t9936, t9937, t9940, t9941, t9942, t9944, t9948) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1488::<F>(t4003, t9768, t9934, t2661, t532, t549, t240, t72, t828, t9400, t595, t66);
    (t9935, t9936, t9937, t9940, t9941, t9942, t9944, t9948)
}
