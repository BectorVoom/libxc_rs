//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1419;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1420;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1421;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta380<F: Float>(t11277: F, t3307: F, t11275: F, t3265: F, t11400: F, t11628: F, t1164: F, t11285: F, t3395: F, t11282: F, t3377: F, t11403: F, t11424: F, t43924: F, t43953: F, t43956: F, t43958: F, t43961: F, t43963: F, t43966: F, t43973: F, t43975: F, t3266: F, t3313: F, t1119: F, t11269: F, t3264: F, t11190: F, t3316: F, t11185: F, t11407: F, t1117: F, t3315: F, t43713: F, t43717: F, t43721: F, t43725: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43748: F, t43750: F, t43754: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43979, t43982, t43984, t43987, t43989) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1419::<F>(t11277, t3307, t11275, t3265, t11400, t11628, t1164, t11285, t3395, t11282, t3377, t11403, t11424);
        let t43990 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1420::<F>(t43924, t43953, t43956, t43958, t43961, t43963, t43966, t43973, t43975, t43979, t43982, t43987, t43989);
        let (t43994, t43997, t44000, t44002, t44006) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1421::<F>(t3266, t3307, t3313, t1119, t11269, t3264, t11190, t3316, t11185, t11407, t1117, t3315);
        let t44021 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1422::<F>(t43713, t43717, t43721, t43725, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43748, t43750, t43754);
    (t43979, t43982, t43984, t43987, t43989, t43990, t43994, t43997, t44000, t44002, t44006, t44021)
}
