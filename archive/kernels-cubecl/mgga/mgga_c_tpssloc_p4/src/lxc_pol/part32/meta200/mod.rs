//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk968;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk969;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk970;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta200<F: Float>(t5250: F, t5335: F, t1825: F, t3901: F, t1380: F, t5287: F, t1338: F, t68: F, t544: F, t1352: F, t1834: F, t5318: F, t553: F, t1332: F, t1336: F, t1381: F, t1383: F, t1814: F, t1838: F, t1840: F, t3777: F, t5230: F, t5234: F, t5334: F, t564: F, t1378: F, t1375: F, t1386: F, t1843: F, t3758: F, t3882: F, t5211: F, t5213: F, t5215: F, t5217: F, t5319: F, t5321: F, t5326: F, t568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5336, t5339, t5341, t5343, t5344) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk968::<F>(t5250, t5335, t1825, t3901, t1380, t5287, t1338, t68, t544);
        let (t5345, t5348, t5349, t5351, t5353) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk969::<F>(t1352, t5335, t1338, t1834, t5318, t553, t1332, t1336, t1381, t1383, t1814, t1838, t1840, t3777, t5230, t5234, t5334, t5336, t5339, t5341, t5344, t544, t564);
        let t5354 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk970::<F>(t1378, t5353);
        let t5356 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk971::<F>(t1375, t1386, t1843, t3758, t3882, t5211, t5213, t5215, t5217, t5319, t5321, t5326, t5354, t568);
    (t5336, t5339, t5341, t5343, t5344, t5345, t5348, t5349, t5351, t5353, t5354, t5356)
}
