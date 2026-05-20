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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta390<F: Float>(t14127: F, t4086: F, t543: F, t2782: F, t1882: F, t4114: F, t2482: F, t122: F, t4003: F, t72: F, t1398: F, t676: F, t10069: F, t5737: F, t10015: F, t10020: F, t10027: F, t10032: F, t10035: F, t10041: F, t10044: F, t14116: F, t14120: F, t14126: F, t4004: F, t5735: F, t5745: F, t9840: F, t5710: F, t1432: F, t686: F, t136: F, t1892: F, t2457: F, t3964: F, t2435: F, t5760: F, t3999: F, t545: F, t869: F, t689: F, t225: F, t9990: F, t213: F, t10062: F, t10130: F, t13805: F, t1399: F, t14122: F, t1883: F, t3924: F, t4057: F, t5675: F, t5755: F, t5767: F, t820: F, t2777: F, t5759: F, t2439: F, t5659: F, t4101: F, t10139: F, t13926: F, t4100: F, t10014: F, t5741: F, t13790: F, t10022: F, t10066: F, t10070: F, t10074: F, t10080: F, t10085: F, t10098: F, t10102: F, t14066: F, t546: F, t786: F, t4104: F, t2470: F, t5740: F, t5763: F, t1385: F, t10105: F, t10109: F, t10114: F, t10117: F, t10120: F, t10126: F, t10129: F, t10137: F, t10143: F, t13921: F, t1437: F, t4118: F, t1427: F, t1904: F, t3899: F, t10151: F, t10154: F, t14091: F, t14096: F, t14097: F, t14102: F, t14105: F, t14108: F, t14111: F, t1424: F, t4132: F, t5715: F, t9695: F, t3920: F, t5603: F, t5718: F, t1893: F, t2453: F, t3908: F, t3895: F, t10157: F, t10160: F, t10163: F, t10166: F, t10169: F, t10176: F, t1445: F, t4071: F, t4078: F, t5775: F, t13750: F, t14088: F, t1343: F, t13664: F, t13667: F, t13669: F, t13671: F, t13673: F, t13674: F, t13682: F, t13683: F, t13716: F, t13885: F, t13886: F, t13888: F, t1450: F, t198: F, t3889: F, t4135: F, t4139: F, t4144: F, t532: F, t5532: F, t5541: F, t5542: F, t9524: F, t9542: F, t9854: F, t9865: F, t9868: F) -> (F, F, F, F, F, F) {
        let (t14131, t14141, t14143, t14144) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1467::<F>(t14127, t4086, t543, t2782, t1882, t4114, t2482, t122, t4003, t72, t1398, t676);
        let t14151 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1468::<F>(t14143, t14144, t14141, t10069, t5737, t10015, t10020, t10027, t10032, t10035, t10041, t10044, t14116, t14120, t14126, t14131, t4004, t5735, t5745, t9840);
        let (t14158, t14161, t14166, t14171, t14188) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1469::<F>(t5710, t72, t1432, t686, t136, t1892, t2457, t3964, t2435, t5760, t3999, t545);
        let t14200 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1470::<F>(t14188, t869, t689, t225, t9990, t213, t10062, t10130, t13805, t1399, t14122, t14127, t14158, t14161, t14166, t14171, t1883, t3924, t4004, t4057, t5675, t5735, t5745, t5755, t5767, t820);
        let (t14203, t14209, t14218) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1471::<F>(t2777, t5759, t2439, t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101);
        let (t14221, t14224, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1472::<F>(t136, t1883, t2457, t10139, t13926, t543, t4100, t2782, t10014, t5741, t13790, t1398);
        let t14237 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1473::<F>(t10022, t14230, t2782, t10066, t10070, t10074, t10080, t10085, t10098, t10102, t14066, t14203, t14209, t14218, t14221, t14227, t14229, t213, t546);
        let t14266 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1474::<F>(t1892, t4086, t786, t4104, t2470, t5740, t4101, t1432, t5763, t1385, t5710, t10105, t10109, t10114, t10117, t10120, t10126, t10129, t10137, t10143, t13921, t1399, t1437, t3924, t4118, t5659, t5767, t820);
        let (t14268, t14269, t14279) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1475::<F>(t14151, t14200, t14237, t14266, t1427, t1904, t3899, t689, t10151, t10154, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t1424, t4132, t5715, t9695);
        let (t14280, t14290, t14294, t14297, t14299) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1476::<F>(t3920, t5603, t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t213, t5710);
        let t14302 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1477::<F>(t10157, t10160, t10163, t10166, t10169, t10176, t14280, t14290, t14294, t14297, t14299, t1445, t4071, t4078, t5715, t5775);
        let (t14304, t14308) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1478::<F>(t13750, t14088, t14279, t14302, t1343, t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13716, t13885, t13886, t13888, t1450, t198, t3889, t4135, t4139, t4144, t532, t5532, t5541, t5542, t9524, t9542, t9854, t9865, t9868);
    (t14224, t14230, t14268, t14269, t14304, t14308)
}
