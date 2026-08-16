//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1852;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta467(t1380: f64, t20568: f64, t1825: f64, t19660: f64, t5348: f64, t6420: f64, t20473: f64, t5335: f64, t20554: f64, t6415: f64, t19657: f64, t16428: f64, t6388: f64, t1336: f64, t1814: f64, t1838: f64, t1840: f64, t19815: f64, t20595: f64, t20616: f64, t20622: f64, t20625: f64, t5234: f64, t5334: f64, t5344: f64, t544: f64, t564: f64, t6378: f64, t6448: f64, t6451: f64, t6454: f64, t6456: f64, t6458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1852(t1380, t20568, t1825, t19660, t5348, t6420, t20473, t5335, t20554, t6415, t19657, t16428, t6388);
        let t20661 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1853(t1336, t1814, t1838, t1840, t19815, t20595, t20616, t20622, t20625, t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651, t5234, t5334, t5344, t544, t564, t6378, t6448, t6451, t6454, t6456, t6458);
    (t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651, t20661)
}
