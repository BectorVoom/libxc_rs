//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta154 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk713;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk714;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta154<F: Float>(t457: F, t6144: F, t460: F, t974: F, t1174: F, t1710: F, t1717: F, t3430: F, t3447: F, t463: F, t4887: F, t4889: F, t4897: F, t4917: F, t6109: F, t6120: F, t6123: F, t6127: F, t6131: F, t6141: F, t491: F, t1720: F, t1751: F, t1730: F, t1743: F, t1417: F, t47: F, t480: F, t479: F, t471: F, t225: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6146, t6147, t6150) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk713::<F>(t457, t6144, t460, t974, t1174, t1710, t1717, t3430, t3447, t463, t4887, t4889, t4897, t4917, t6109, t6120, t6123, t6127, t6131, t6141);
        let (t6151, t6153, t6158, t6163) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk714::<F>(t491, t6150, t1720, t1751, t1730, t1743, t1417, t47, t480);
        let (t6164, t6165, t6168, t6169) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk715::<F>(t479, t6163, t471, t225, t6150, t68);
    (t6146, t6147, t6150, t6151, t6153, t6158, t6163, t6164, t6165, t6168, t6169)
}
