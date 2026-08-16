//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2544;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta710<F: Float>(t10403: F, t10422: F, t14214: F, t3030: F, t4552: F, t3032: F, t3129: F, t13998: F, t2960: F, t42875: F, t4338: F, t973: F, t14040: F, t3070: F, t10516: F, t4640: F, t14121: F, t13748: F, t13965: F, t3114: F, t14202: F, t3117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49629, t49649, t49650, t49651, t49658, t49661) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2544::<F>(t10403, t10422, t14214, t3030, t4552, t3032, t3129, t13998, t2960, t42875, t4338, t973);
        let (t49666, t49678, t49682, t49684, t49690, t49692) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2545::<F>(t10422, t14040, t3070, t10516, t4640, t10403, t14121, t13748, t2960, t13965, t3114, t14202, t3117);
    (t49629, t49649, t49650, t49651, t49658, t49661, t49666, t49678, t49682, t49684, t49690, t49692)
}
