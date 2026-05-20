//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta681 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta681<F: Float>(t12800: F, t3636: F, t3551: F, t3565: F, t225: F, t12884: F, t828: F, t12788: F, t3625: F, t12732: F, t73: F, t3555: F, t3766: F, t5330: F) -> (F, F, F, F, F, F, F) {
        let (t44418, t44420, t44421, t44425, t44427, t44431, t44484) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2494::<F>(t12800, t3636, t3551, t3565, t225, t12884, t828, t12788, t3625, t12732, t73, t3555, t3766, t5330);
    (t44418, t44420, t44421, t44425, t44427, t44431, t44484)
}
