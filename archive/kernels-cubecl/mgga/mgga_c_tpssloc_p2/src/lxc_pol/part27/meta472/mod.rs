//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1835;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1836;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta472<F: Float>(t23477: F, t23479: F, t6721: F, t6739: F, t6741: F, t1937: F, t23447: F, t23449: F, t23454: F, t23457: F, t23460: F, t23463: F, t23465: F, t23469: F, t23474: F, t350: F, t378: F, t6747: F, t344: F, t6729: F, t6740: F, t3008: F, t343: F, t6734: F, t3103: F, t6755: F, t3120: F, t360: F, t68: F, t6744: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23480, t23482) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1835::<F>(t23477, t23479, t6721, t6739);
        let (t23483, t23486) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1836::<F>(t23482, t6741, t1937, t23447, t23449, t23454, t23457, t23460, t23463, t23465, t23469, t23474, t23480, t350, t378, t6747);
        let (t23488, t23489, t23494, t23495, t23500, t23503, t23504) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1837::<F>(t344, t6729, t6740, t3008, t343, t6734, t3103, t6755, t3120, t360, t68, t6744);
    (t23480, t23482, t23483, t23486, t23488, t23489, t23494, t23495, t23500, t23503, t23504)
}
