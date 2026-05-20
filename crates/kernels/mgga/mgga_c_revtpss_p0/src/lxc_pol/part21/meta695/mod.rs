//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta695 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2517;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta695<F: Float>(t225: F, t45551: F, t1209: F, t13107: F, t460: F, t13038: F, t42859: F, t44531: F, t473: F, t17879: F, t17845: F, t17852: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t45552, t45568, t45575, t45607, t45608, t45618, t45619, t45624, t45634, t45654, t45659) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2517::<F>(t225, t45551, t1209, t13107, t460, t13038, t42859, t44531, t473, t17879, t17845, t17852);
    (t45552, t45568, t45575, t45607, t45608, t45618, t45619, t45624, t45634, t45654, t45659)
}
