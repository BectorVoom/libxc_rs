//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk920;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk921;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta221<F: Float>(t2884: F, t307: F, t302: F, t10743: F, t2888: F, t10294: F, t10544: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t10530: F, t10538: F, t10547: F, t10550: F, t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10591: F, t10597: F, t10600: F, t932: F, t922: F, t2887: F, t310: F, t2791: F, t888: F, t2794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10770, t10771, t10772, t10789) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk920::<F>(t2884, t307, t302, t10743, t2888, t10294, t10544, t10296, t10298, t10300, t10302, t10307, t10314, t10320, t10323, t10530, t10538, t10547, t10550);
        let t10804 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk921::<F>(t10311, t10318, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575, t10589, t10591, t10597, t10600);
        let (t10805, t10806, t10810, t10811, t10813, t10814, t10817, t10819) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk922::<F>(t10789, t10804, t932, t2884, t922, t302, t2887, t310, t10743, t2791, t888, t2794);
    (t10770, t10771, t10772, t10805, t10806, t10810, t10811, t10813, t10814, t10817, t10819)
}
