//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1303;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1304;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta233<F: Float>(t731: F, t9751: F, t746: F, t9490: F, t172: F, t9489: F, t9493: F, t9720: F, t2512: F, t9711: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F, t702: F, t683: F, t9731: F, t2405: F, t2420: F, t703: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9777) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1303::<F>(t731, t9751, t746, t9490, t172, t9489, t9493, t9720, t2512, t9711, t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
        let (t9778, t9780) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1304::<F>(t702, t9777, t683);
        let (t9781, t9789) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1305::<F>(t731, t9731, t2405, t2420, t703);
    (t9752, t9755, t9758, t9759, t9762, t9763, t9766, t9777, t9778, t9780, t9781, t9789)
}
