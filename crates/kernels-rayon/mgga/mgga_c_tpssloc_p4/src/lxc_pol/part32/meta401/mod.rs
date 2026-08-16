//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1516;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1517;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1518;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1519;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1520;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1521;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1522;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1523;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1524;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1525;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta401(t135: f64, t5889: f64, t973: f64, t5893: f64, t5884: f64, t4593: f64, t4650: f64, t4582: f64, t5398: f64, t607: f64, t4583: f64, t1041: f64, t13948: f64, t13952: f64, t13959: f64, t13963: f64, t13966: f64, t13972: f64, t2960: f64, t3039: f64, t5885: f64, t5890: f64, t5894: f64, t4588: f64, t1023: f64, t5681: f64, t3071: f64, t248: f64, t3101: f64, t5878: f64, t3051: f64, t5685: f64, t4630: f64, t4641: f64, t5873: f64, t3130: f64, t376: f64, t5872: f64, t1022: f64, t10482: f64, t1539: f64, t5867: f64, t884: f64, t10390: f64, t10480: f64, t10904: f64, t13995: f64, t14000: f64, t14027: f64, t3070: f64, t4575: f64, t5875: f64, t5909: f64, t5392: f64, t14172: f64, t1409: f64, t3966: f64, t14187: f64, t1616: f64, t4347: f64, t5866: f64, t4594: f64, t10413: f64, t10436: f64, t10511: f64, t10871: f64, t14049: f64, t14059: f64, t3114: f64, t4585: f64, t4590: f64, t4644: f64, t5869: f64, t3131: f64, t4649: f64, t16558: f64, t998: f64, t974: f64, t13835: f64, t4531: f64, t13769: f64, t13839: f64, t6733: f64, t4540: f64, t7577: f64, t4546: f64, t343: f64, t5842: f64, t984: f64, t2970: f64, t5824: f64, t10226: f64, t13782: f64, t13787: f64, t13790: f64, t13825: f64, t2986: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17616, t17621, t17625, t17632, t17635) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1516(t135, t5889, t973, t5893, t5884, t4593, t4650, t4582, t5398, t607);
        let t17640 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1517(t17635, t4583, t4582, t1041, t13948, t13952, t13959, t13963, t13966, t13972, t17616, t17621, t17625, t17632, t2960, t3039, t5885, t5890, t5894);
        let (t17643, t17649, t17656, t17659) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1518(t17635, t4588, t4582, t1023, t5681, t3071, t248, t3101, t5878, t3039, t3051, t5685);
        let (t17660, t17662, t17668, t17670, t17671) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1519(t1041, t17659, t4630, t4641, t248, t3101, t5873, t3130, t376, t5872, t1022, t10482);
        let t17684 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1520(t17670, t17671, t4582, t1539, t4650, t3071, t5867, t884, t10390, t1041, t10480, t10904, t13995, t14000, t14027, t17643, t17649, t17656, t17660, t17662, t17668, t3070, t4575, t5875, t5909);
        let t17686 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1521(t5392, t607);
        let (t17688, t17691) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1522(t14172, t17686, t4582, t1409, t3966);
        let (t17693, t17697, t17701, t17705, t17712) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1523(t17691, t4588, t4582, t14187, t17686, t5878, t884, t3071, t1616, t4347, t376, t5866);
        let t17725 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1524(t17712, t4594, t4582, t1023, t1041, t10413, t10436, t10511, t10871, t14049, t14059, t17688, t17693, t17697, t17701, t17705, t3039, t3070, t3114, t3130, t4585, t4590, t4644, t5869);
        let (t17734, t17738, t17742, t17745, t17748) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1525(t3131, t4649, t4593, t4582, t16558, t998, t974, t13835, t4531, t13769, t13839, t1539, t6733);
        let t17766 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1526(t17748, t4531, t4540, t7577, t4546, t343, t5842, t984, t2970, t5824, t973, t10226, t13782, t13787, t13790, t13825, t17742, t17745, t2960, t2986, t5825);
    (t17635, t17640, t17670, t17684, t17686, t17691, t17725, t17734, t17738, t17766)
}
