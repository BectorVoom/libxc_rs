//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta390 (260520-c91 hierarchical CSE).
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
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1467;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1468;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1469;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1470;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1471;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1472;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1473;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1474;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1475;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1476;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1477;
use chunk11::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta390(t14127: f64, t4086: f64, t543: f64, t2782: f64, t1882: f64, t4114: f64, t2482: f64, t122: f64, t4003: f64, t72: f64, t1398: f64, t676: f64, t10069: f64, t5737: f64, t10015: f64, t10020: f64, t10027: f64, t10032: f64, t10035: f64, t10041: f64, t10044: f64, t14116: f64, t14120: f64, t14126: f64, t4004: f64, t5735: f64, t5745: f64, t9840: f64, t5710: f64, t1432: f64, t686: f64, t136: f64, t1892: f64, t2457: f64, t3964: f64, t2435: f64, t5760: f64, t3999: f64, t545: f64, t869: f64, t689: f64, t225: f64, t9990: f64, t213: f64, t10062: f64, t10130: f64, t13805: f64, t1399: f64, t14122: f64, t1883: f64, t3924: f64, t4057: f64, t5675: f64, t5755: f64, t5767: f64, t820: f64, t2777: f64, t5759: f64, t2439: f64, t5659: f64, t4101: f64, t10139: f64, t13926: f64, t4100: f64, t10014: f64, t5741: f64, t13790: f64, t10022: f64, t10066: f64, t10070: f64, t10074: f64, t10080: f64, t10085: f64, t10098: f64, t10102: f64, t14066: f64, t546: f64, t786: f64, t4104: f64, t2470: f64, t5740: f64, t5763: f64, t1385: f64, t10105: f64, t10109: f64, t10114: f64, t10117: f64, t10120: f64, t10126: f64, t10129: f64, t10137: f64, t10143: f64, t13921: f64, t1437: f64, t4118: f64, t1427: f64, t1904: f64, t3899: f64, t10151: f64, t10154: f64, t14091: f64, t14096: f64, t14097: f64, t14102: f64, t14105: f64, t14108: f64, t14111: f64, t1424: f64, t4132: f64, t5715: f64, t9695: f64, t3920: f64, t5603: f64, t5718: f64, t1893: f64, t2453: f64, t3908: f64, t3895: f64, t10157: f64, t10160: f64, t10163: f64, t10166: f64, t10169: f64, t10176: f64, t1445: f64, t4071: f64, t4078: f64, t5775: f64, t13750: f64, t14088: f64, t1343: f64, t13664: f64, t13667: f64, t13669: f64, t13671: f64, t13673: f64, t13674: f64, t13682: f64, t13683: f64, t13716: f64, t13885: f64, t13886: f64, t13888: f64, t1450: f64, t198: f64, t3889: f64, t4135: f64, t4139: f64, t4144: f64, t532: f64, t5532: f64, t5541: f64, t5542: f64, t9524: f64, t9542: f64, t9854: f64, t9865: f64, t9868: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t14131, t14141, t14143, t14144) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1467(t14127, t4086, t543, t2782, t1882, t4114, t2482, t122, t4003, t72, t1398, t676);
        let t14151 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1468(t14143, t14144, t14141, t10069, t5737, t10015, t10020, t10027, t10032, t10035, t10041, t10044, t14116, t14120, t14126, t14131, t4004, t5735, t5745, t9840);
        let (t14158, t14161, t14166, t14171, t14188) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1469(t5710, t72, t1432, t686, t136, t1892, t2457, t3964, t2435, t5760, t3999, t545);
        let t14200 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1470(t14188, t869, t689, t225, t9990, t213, t10062, t10130, t13805, t1399, t14122, t14127, t14158, t14161, t14166, t14171, t1883, t3924, t4004, t4057, t5675, t5735, t5745, t5755, t5767, t820);
        let (t14203, t14209, t14218) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1471(t2777, t5759, t2439, t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101);
        let (t14221, t14224, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1472(t136, t1883, t2457, t10139, t13926, t543, t4100, t2782, t10014, t5741, t13790, t1398);
        let t14237 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1473(t10022, t14230, t2782, t10066, t10070, t10074, t10080, t10085, t10098, t10102, t14066, t14203, t14209, t14218, t14221, t14227, t14229, t213, t546);
        let t14266 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1474(t1892, t4086, t786, t4104, t2470, t5740, t4101, t1432, t5763, t1385, t5710, t10105, t10109, t10114, t10117, t10120, t10126, t10129, t10137, t10143, t13921, t1399, t1437, t3924, t4118, t5659, t5767, t820);
        let (t14268, t14269, t14279) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1475(t14151, t14200, t14237, t14266, t1427, t1904, t3899, t689, t10151, t10154, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t1424, t4132, t5715, t9695);
        let (t14280, t14290, t14294, t14297, t14299) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1476(t3920, t5603, t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t213, t5710);
        let t14302 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1477(t10157, t10160, t10163, t10166, t10169, t10176, t14280, t14290, t14294, t14297, t14299, t1445, t4071, t4078, t5715, t5775);
        let (t14304, t14308) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1478(t13750, t14088, t14279, t14302, t1343, t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13716, t13885, t13886, t13888, t1450, t198, t3889, t4135, t4139, t4144, t532, t5532, t5541, t5542, t9524, t9542, t9854, t9865, t9868);
    (t14224, t14230, t14268, t14269, t14304, t14308)
}
