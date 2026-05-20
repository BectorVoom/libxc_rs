//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2336;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta605<F: Float>(t10638: F, t251: F, t10111: F, t22: F, t2789: F, t588: F, t870: F, t10963: F, t9303: F, t10069: F, t10934: F, t10518: F, t10542: F, t10612: F, t2398: F, t2434: F, t2626: F, t2629: F, t676: F, t9425: F, t2567: F, t2576: F, t2582: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39714, t39719, t39723, t39724, t39726, t39731) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2336::<F>(t10638, t251, t10111, t22, t2789, t588, t870, t10963, t9303, t10069, t10934, t10518, t10542);
        let (t39737, t39739, t39741, t39742, t39744, t39747) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2337::<F>(t10612, t2398, t2434, t2626, t2629, t676, t9425, t2567, t2576, t2582);
    (t39714, t39719, t39723, t39724, t39726, t39731, t39737, t39739, t39741, t39742, t39744, t39747)
}
