//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1743;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1744;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta504<F: Float>(t27074: F, t5250: F, t1352: F, t26393: F, t1825: F, t24116: F, t26406: F, t1336: F, t22707: F, t24099: F, t26379: F, t26381: F, t26386: F, t26390: F, t26398: F, t26412: F, t26416: F, t26419: F, t26424: F, t26427: F, t3777: F, t5234: F, t5334: F, t5344: F, t7209: F, t7932: F, t26429: F, t1338: F, t7918: F, t5287: F, t7208: F, t27051: F, t553: F, t1332: F, t1814: F, t2089: F, t22728: F, t22731: F, t22746: F, t22753: F, t22896: F, t24108: F, t24110: F, t26434: F, t26437: F, t26449: F, t26463: F, t26468: F, t5230: F, t544: F, t7211: F, t7934: F, t1378: F, t1375: F, t1386: F, t16022: F, t16439: F, t1843: F, t2092: F, t22676: F, t24095: F, t26475: F, t27067: F, t27068: F, t27070: F, t3758: F, t3882: F, t5215: F, t5321: F, t568: F, t7199: F, t7214: F, t7937: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t27075, t27078, t27086, t27095) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1743::<F>(t27074, t5250, t1352, t26393, t1825, t24116, t26406, t1336, t22707, t24099, t26379, t26381, t26386, t26390, t26398, t26412, t26416, t26419, t26424, t26427, t3777, t5234, t5334, t5344, t7209, t7932);
        let (t27097, t27098, t27103, t27105, t27113) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1744::<F>(t26429, t1338, t7918, t1352, t5287, t7208, t27051, t553, t1332, t1336, t1814, t2089, t22728, t22731, t22746, t22753, t22896, t24108, t24110, t26434, t26437, t26449, t26463, t26468, t5230, t544, t7211, t7934);
        let (t27114, t27115, t27127) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1745::<F>(t27095, t27113, t1378, t1375, t1386, t16022, t16439, t1843, t2092, t22676, t24095, t26475, t27067, t27068, t27070, t3758, t3882, t5215, t5321, t568, t7199, t7214, t7937);
    (t27075, t27078, t27086, t27097, t27098, t27103, t27105, t27114, t27115, t27127)
}
