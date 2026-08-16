//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1069;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1070;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1071;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta293<F: Float>(t12437: F, t1378: F, t12237: F, t562: F, t12434: F, t539: F, t225: F, t3755: F, t12016: F, t12023: F, t12027: F, t12030: F, t12033: F, t12036: F, t1375: F, t1386: F, t3758: F, t3882: F, t3889: F, t3912: F, t568: F, t1388: F, t3698: F, t3700: F, t570: F, t11976: F, t11978: F, t11980: F, t11982: F, t11984: F, t12012: F, t12044: F, t12046: F, t12156: F, t1297: F, t1390: F, t193: F, t533: F, t571: F, t9457: F, t9476: F, t9484: F, t9780: F, t3914: F, t3719: F, t12048: F, t12051: F, t12053: F, t12055: F, t12057: F, t12059: F, t12085: F, t12087: F, t12090: F, t12092: F, t12094: F, t1307: F, t3918: F, t5126: F, t9789: F, t9793: F, t12098: F, t12101: F, t12103: F, t12105: F, t12107: F, t12109: F, t12112: F, t12114: F, t12116: F, t12118: F, t12121: F, t12123: F, t9797: F, t9820: F, t9824: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12438, t12440, t12442, t12444, t12451) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1069::<F>(t12437, t1378, t12237, t562, t12434, t539, t225, t3755, t12016, t12023, t12027, t12030, t12033, t12036, t1375, t1386, t3758, t3882, t3889, t3912, t568);
        let (t12458, t12461, t12465) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1070::<F>(t1388, t3698, t3700, t570, t11976, t11978, t11980, t11982, t11984, t12012, t12044, t12046, t12156, t12451, t1297, t1390, t193, t533, t571, t9457, t9476, t9484, t9780);
        let (t12466, t12474) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1071::<F>(t1390, t3914, t3719, t571, t12048, t12051, t12053, t12055, t12057, t12059, t12085, t12087, t12090, t12092, t12094, t1307, t3918, t5126, t9789, t9793);
        let t12476 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1072::<F>(t12098, t12101, t12103, t12105, t12107, t12109, t12112, t12114, t12116, t12118, t12121, t12123, t9797, t9820, t9824);
    (t12438, t12440, t12442, t12444, t12451, t12458, t12461, t12465, t12466, t12474, t12476)
}
