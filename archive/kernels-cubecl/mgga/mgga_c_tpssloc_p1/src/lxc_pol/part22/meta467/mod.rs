//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1852;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta467<F: Float>(t1380: F, t20568: F, t1825: F, t19660: F, t5348: F, t6420: F, t20473: F, t5335: F, t20554: F, t6415: F, t19657: F, t16428: F, t6388: F, t1336: F, t1814: F, t1838: F, t1840: F, t19815: F, t20595: F, t20616: F, t20622: F, t20625: F, t5234: F, t5334: F, t5344: F, t544: F, t564: F, t6378: F, t6448: F, t6451: F, t6454: F, t6456: F, t6458: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1852::<F>(t1380, t20568, t1825, t19660, t5348, t6420, t20473, t5335, t20554, t6415, t19657, t16428, t6388);
        let t20661 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1853::<F>(t1336, t1814, t1838, t1840, t19815, t20595, t20616, t20622, t20625, t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651, t5234, t5334, t5344, t544, t564, t6378, t6448, t6451, t6454, t6456, t6458);
    (t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651, t20661)
}
