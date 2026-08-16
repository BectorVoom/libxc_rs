//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1042;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1043;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1044;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1045;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1046;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1047;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta160<F: Float>(t1365: F, t67: F, t246: F, t120: F, t1351: F, t1307: F, t550: F, t1291: F, t2663: F, t1284: F, t758: F, t2408: F, t2417: F, t2426: F, t2486: F, t3683: F, t3688: F, t3690: F, t3693: F, t3695: F, t2225: F, t522: F, t2221: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3804, t3805) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1042::<F>(t1365, t67, t246);
        let (t3806, t3807) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1043::<F>(t120, t1351, t1307, t550);
        let t3809 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1044::<F>(t3805, t3806, t3807);
        let t3813 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1045::<F>(t1291, t2663);
        let (t3814, t3815, t3816, t3817) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1046::<F>(t1284, t67, t758, t2408, t2417, t2426, t2486, t3683, t3688, t3690, t3693, t3695, t3813);
        let t3819 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1047::<F>(t2225, t522);
        let t3821 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1048::<F>(t2221, t522);
    (t3804, t3805, t3807, t3809, t3813, t3814, t3815, t3816, t3817, t3819, t3821)
}
