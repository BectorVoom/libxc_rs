//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta373 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1418;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1419;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1420;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1421;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1422;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1423;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1424;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1425;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1426;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1427;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1428;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta373<F: Float>(t4680: F, t4684: F, t11060: F, t3040: F, t1629: F, t4673: F, t1049: F, t4649: F, t1060: F, t11066: F, t1615: F, t3166: F, t4677: F, t1625: F, t3120: F, t14506: F, t3199: F, t1058: F, t11034: F, t11051: F, t11059: F, t11065: F, t14572: F, t1630: F, t1632: F, t3076: F, t3180: F, t3186: F, t3193: F, t3200: F, t3202: F, t4669: F, t4674: F, t4678: F, t4681: F, t3185: F, t1932: F, t360: F, t3201: F, t6739: F, t14526: F, t383: F, t1022: F, t4657: F, t3188: F, t1057: F, t14205: F, t11054: F, t1003: F, t1061: F, t1063: F, t11037: F, t11046: F, t13940: F, t1610: F, t3189: F, t3197: F, t3204: F, t353: F, t384: F, t4615: F, t4685: F, t4689: F, t4691: F, t1055: F, t10160: F, t10170: F, t1052: F, t1066: F, t11010: F, t14545: F, t14549: F, t14552: F, t14555: F, t14562: F, t1635: F, t3169: F, t3176: F, t3207: F, t388: F, t4557: F, t4660: F, t4665: F, t14543: F, t1068: F, t1070: F, t13510: F, t13512: F, t13514: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t13661: F, t13665: F, t13666: F, t13720: F, t13722: F, t13726: F, t13729: F, t13731: F, t13734: F, t193: F, t336: F, t4700: F, t11094: F, t1637: F, t14257: F, t14262: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14424: F, t14472: F, t14475: F, t14477: F, t14479: F, t14482: F, t14484: F, t14486: F, t3209: F, t3213: F, t4701: F, t25: F, t265: F, t394: F, t13493: F, t1074: F, t12606: F, t13503: F, t13504: F, t13506: F, t1408: F, t1409: F, t1534: F, t1642: F, t2249: F, t2250: F, t2756: F, t3220: F, t396: F, t3966: F, t40: F, t4324: F, t4705: F, t606: F, t607: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3640: F, t5091: F, t3415: F, t4869: F, t1654: F, t2394: F, t4734: F, t690: F, t1089: F, t1088: F, t123: F) -> (F, F, F, F, F, F, F, F) {
        let (t14574, t14578, t14581, t14587, t14591, t14595) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1418::<F>(t4680, t4684, t11060, t3040, t1629, t4673, t1049, t4649, t1060, t11066, t1615, t3166);
        let t14613 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1419::<F>(t1060, t14595, t4673, t4677, t1625, t3120, t14506, t3199, t1058, t11034, t11051, t11059, t11065, t14572, t14574, t14578, t14581, t14587, t14591, t1630, t1632, t3076, t3180, t3186, t3193, t3200, t3202, t4669, t4674, t4678, t4681);
        let (t14615, t14618, t14623, t14626, t14627, t14630) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1420::<F>(t4677, t4684, t14506, t3185, t1932, t3120, t360, t1629, t1625, t3040, t3201, t6739);
        let (t14631, t14640, t14645, t14648, t14651, t14654) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1421::<F>(t14630, t1629, t14526, t383, t1022, t4657, t1060, t14626, t3188, t1057, t14205, t11054);
        let t14657 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1422::<F>(t1003, t1058, t1061, t1063, t11037, t11046, t13940, t14615, t14618, t14623, t14627, t14631, t14640, t14645, t14648, t14651, t14654, t1610, t3180, t3186, t3189, t3197, t3200, t3204, t353, t384, t4615, t4669, t4685, t4689, t4691);
        let t14661 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1423::<F>(t14613, t14657, t1055, t10160, t10170, t1052, t1066, t11010, t14545, t14549, t14552, t14555, t14562, t1635, t3169, t3176, t3207, t388, t4557, t4660, t4665);
        let t14666 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1424::<F>(t14543, t14661, t1068, t1070, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13666, t13720, t13722, t13726, t13729, t13731, t13734, t193, t336, t4700);
        let t14673 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1425::<F>(t11094, t1637, t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479, t14482, t14484, t14486, t3209, t3213, t4700, t4701);
        let t14687 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1426::<F>(t25, t265, t394, t13493, t14666, t14673, t1074, t12606, t13503, t13504, t13506, t1408, t1409, t1534, t1642, t2249, t2250, t2756, t3220, t396, t3966, t40, t4324, t4705, t606, t607, dens_threshold, rho0, zeta_threshold);
        let (t14696, t14701, t14702) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1427::<F>(t3640, t5091, t3415, t4869, t1654, t2394);
        let t14704 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1428::<F>(t4734, t690);
        let (t14705, t14706, t14708) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1429::<F>(t14704, t1089, t12606, t1088, t123);
    (t14687, t14696, t14701, t14702, t14704, t14705, t14706, t14708)
}
