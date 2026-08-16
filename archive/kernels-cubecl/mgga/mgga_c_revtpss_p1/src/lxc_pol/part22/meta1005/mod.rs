//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1005 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3436;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1005<F: Float>(t15573: F, t4719: F, t11524: F, t19133: F, t981: F, t15526: F, t19134: F, t3022: F, t15266: F, t52894: F, t63597: F, t19021: F, t3011: F, t4733: F, t19049: F, t3034: F, t19045: F, t300: F, t983: F, t63940: F, t63943: F, t64327: F, t64329: F, t64488: F, t64491: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t64493, t64496, t64498, t64500, t64503, t64504) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3436::<F>(t15573, t4719, t11524, t19133, t981, t15526, t19134, t3022, t15266, t52894, t63597, t19021, t3011);
        let (t64507, t64509, t64512, t64513) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3437::<F>(t4733, t64504, t981, t19049, t3034, t19045, t300, t983, t63940, t63943, t64327, t64329, t64488, t64491, t64493, t64496, t64498, t64500, t64503);
    (t64493, t64496, t64498, t64500, t64503, t64507, t64509, t64512, t64513)
}
