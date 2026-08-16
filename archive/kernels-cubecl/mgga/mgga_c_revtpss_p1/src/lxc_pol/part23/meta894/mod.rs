//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta894 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2851;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta894<F: Float>(t61130: F, t10439: F, t22688: F, t750: F, t49926: F, t18263: F, t4308: F, t49940: F, t23211: F, t72: F, t757: F, t61165: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t39770: F, t39773: F, t49930: F) -> (F, F, F, F, F, F, F, F) {
        let (t76963, t76966, t76967, t76969, t76970, t76973, t76974) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2851::<F>(t61130, t10439, t22688, t750, t49926, t18263, t4308, t49940, t23211, t72, t757, t61165);
        let t76975 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2852::<F>(t39741, t39744, t39747, t39750, t39756, t39760, t39764, t39770, t39773, t49930, t76963, t76966, t76967, t76969, t76970, t76973, t76974);
    (t76963, t76966, t76967, t76969, t76970, t76973, t76974, t76975)
}
