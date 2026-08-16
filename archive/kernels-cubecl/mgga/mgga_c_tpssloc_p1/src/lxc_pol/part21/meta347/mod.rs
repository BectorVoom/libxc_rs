//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1745;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1746;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta347<F: Float>(t221: F, t2379: F, t4128: F, t1489: F, t9541: F, t4126: F, t782: F, t4130: F, t12971: F, t210: F, t214: F, t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F, t2576: F, t13005: F, t787: F, t9572: F, t9574: F, t9579: F, t9583: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13007, t13010, t13012) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1745::<F>(t221, t2379, t4128, t1489, t9541, t4126, t782);
        let (t13014, t13017, t13020, t13022, t13025) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1746::<F>(t13012, t4130, t12971, t210, t214, t2563, t4138, t4134, t9546, t118, t4119, t794);
        let (t13027, t13028) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1747::<F>(t13025, t2576, t13005, t13007, t13010, t13014, t13017, t13020, t13022, t787, t9572, t9574, t9579, t9583);
    (t13007, t13010, t13012, t13014, t13017, t13020, t13022, t13025, t13027, t13028)
}
