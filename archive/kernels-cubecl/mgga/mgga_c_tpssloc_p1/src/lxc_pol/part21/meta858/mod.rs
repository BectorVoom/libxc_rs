//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta858 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3115;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta858<F: Float>(t14961: F, t4869: F, t18915: F, t3415: F, t14858: F, t4875: F, t15838: F, t19267: F, t3633: F, t4700: F, t63280: F, t64446: F, t64447: F, t64454: F, t64456: F, t64458: F, t64460: F, t64462: F, t64464: F, t18918: F, t3411: F, t1703: F, t51807: F, t4879: F, t15036: F, t1155: F, t4857: F, t4861: F, t51848: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t64466, t64470, t64472, t64473) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3115::<F>(t14961, t4869, t18915, t3415, t14858, t4875, t15838, t19267, t3633, t4700, t63280, t64446, t64447, t64454, t64456, t64458, t64460, t64462, t64464);
        let (t64475, t64477, t64479, t64481, t64482, t64485) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3116::<F>(t18918, t3411, t1703, t51807, t14858, t4879, t15036, t4869, t1155, t4857, t4861, t51848);
    (t64466, t64470, t64472, t64473, t64475, t64477, t64479, t64481, t64482, t64485)
}
