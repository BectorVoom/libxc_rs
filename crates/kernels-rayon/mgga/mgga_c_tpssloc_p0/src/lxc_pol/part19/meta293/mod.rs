//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1069;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1070;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1071;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta293(t12437: f64, t1378: f64, t12237: f64, t562: f64, t12434: f64, t539: f64, t225: f64, t3755: f64, t12016: f64, t12023: f64, t12027: f64, t12030: f64, t12033: f64, t12036: f64, t1375: f64, t1386: f64, t3758: f64, t3882: f64, t3889: f64, t3912: f64, t568: f64, t1388: f64, t3698: f64, t3700: f64, t570: f64, t11976: f64, t11978: f64, t11980: f64, t11982: f64, t11984: f64, t12012: f64, t12044: f64, t12046: f64, t12156: f64, t1297: f64, t1390: f64, t193: f64, t533: f64, t571: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64, t3914: f64, t3719: f64, t12048: f64, t12051: f64, t12053: f64, t12055: f64, t12057: f64, t12059: f64, t12085: f64, t12087: f64, t12090: f64, t12092: f64, t12094: f64, t1307: f64, t3918: f64, t5126: f64, t9789: f64, t9793: f64, t12098: f64, t12101: f64, t12103: f64, t12105: f64, t12107: f64, t12109: f64, t12112: f64, t12114: f64, t12116: f64, t12118: f64, t12121: f64, t12123: f64, t9797: f64, t9820: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12438, t12440, t12442, t12444, t12451) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1069(t12437, t1378, t12237, t562, t12434, t539, t225, t3755, t12016, t12023, t12027, t12030, t12033, t12036, t1375, t1386, t3758, t3882, t3889, t3912, t568);
        let (t12458, t12461, t12465) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1070(t1388, t3698, t3700, t570, t11976, t11978, t11980, t11982, t11984, t12012, t12044, t12046, t12156, t12451, t1297, t1390, t193, t533, t571, t9457, t9476, t9484, t9780);
        let (t12466, t12474) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1071(t1390, t3914, t3719, t571, t12048, t12051, t12053, t12055, t12057, t12059, t12085, t12087, t12090, t12092, t12094, t1307, t3918, t5126, t9789, t9793);
        let t12476 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1072(t12098, t12101, t12103, t12105, t12107, t12109, t12112, t12114, t12116, t12118, t12121, t12123, t9797, t9820, t9824);
    (t12438, t12440, t12442, t12444, t12451, t12458, t12461, t12465, t12466, t12474, t12476)
}
