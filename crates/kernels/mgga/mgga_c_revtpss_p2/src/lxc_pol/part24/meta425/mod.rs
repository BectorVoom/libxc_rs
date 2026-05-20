//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta425<F: Float>(t44951: F, t5330: F, t3362: F, t404: F, t43766: F, t13026: F, t43776: F, t43813: F, t3450: F, t3475: F, t426: F, t43816: F) -> (F, F, F, F, F, F, F) {
        let (t44952, t44959, t44974, t45000, t45085, t45106, t45107) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1375::<F>(t44951, t5330, t3362, t404, t43766, t13026, t43776, t43813, t3450, t3475, t426, t43816);
    (t44952, t44959, t44974, t45000, t45085, t45106, t45107)
}
