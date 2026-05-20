//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta777 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta777<F: Float>(t58145: F, t58225: F, t3432: F, t5060: F, t12226: F, t1719: F, t56228: F, t56176: F, t56183: F, t12555: F, t5180: F, t12486: F, t300: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t58411, t58452, t58466, t58473, t58536, t58543, t58607, t58609, t58624, t58647, t58665) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2581::<F>(t58145, t58225, t3432, t5060, t12226, t1719, t56228, t56176, t56183, t12555, t5180, t12486, t300);
    (t58411, t58452, t58466, t58473, t58536, t58543, t58607, t58609, t58624, t58647, t58665)
}
