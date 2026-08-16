//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta684<F: Float>(t45384: F, t487: F, t13180: F, t493: F, t225: F, t13038: F, t42859: F, t460: F, t44531: F, t1209: F, t17879: F, t17845: F) -> (F, F, F, F, F, F, F, F) {
        let (t45449, t45552, t45607, t45608, t45618, t45619, t45634, t45654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2425::<F>(t45384, t487, t13180, t493, t225, t13038, t42859, t460, t44531, t1209, t17879, t17845);
    (t45449, t45552, t45607, t45608, t45618, t45619, t45634, t45654)
}
