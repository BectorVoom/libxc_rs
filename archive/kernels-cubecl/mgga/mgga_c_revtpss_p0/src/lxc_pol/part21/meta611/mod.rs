//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2362;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta611<F: Float>(t2523: F, t9318: F, t2596: F, t746: F, t9385: F, t760: F, t186: F, t2698: F, t685: F, t755: F, t10558: F, t177: F, t762: F, t2491: F, t2495: F, t39871: F, t10326: F, t706: F, t750: F, t9419: F, t72: F, t757: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40094, t40097, t40099, t40101, t40103, t40108) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2362::<F>(t2523, t9318, t2596, t746, t9385, t760, t186, t2698, t685, t755, t10558, t177, t762);
        let (t40113, t40115, t40119, t40121, t40125) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2363::<F>(t2491, t2495, t39871, t760, t10326, t706, t750, t2523, t9419, t10558, t72, t757);
    (t40094, t40097, t40099, t40101, t40103, t40108, t40113, t40115, t40119, t40121, t40125)
}
