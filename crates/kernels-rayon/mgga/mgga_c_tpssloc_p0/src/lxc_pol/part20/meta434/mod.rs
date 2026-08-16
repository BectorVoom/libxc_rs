//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1855;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1856;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1857;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1858;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1859;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1860;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1861;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta434(t1003: f64, t1058: f64, t1061: f64, t1063: f64, t11037: f64, t11046: f64, t13940: f64, t14615: f64, t14618: f64, t14623: f64, t14627: f64, t14631: f64, t14640: f64, t14645: f64, t14648: f64, t14651: f64, t14654: f64, t1610: f64, t3180: f64, t3186: f64, t3189: f64, t3197: f64, t3200: f64, t3204: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4685: f64, t4689: f64, t4691: f64, t14613: f64, t1055: f64, t10160: f64, t10170: f64, t1052: f64, t1066: f64, t11010: f64, t14545: f64, t14549: f64, t14552: f64, t14555: f64, t14562: f64, t1635: f64, t3169: f64, t3176: f64, t3207: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t14543: f64, t1068: f64, t1070: f64, t13510: f64, t13512: f64, t13514: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t13661: f64, t13665: f64, t13666: f64, t13720: f64, t13722: f64, t13726: f64, t13729: f64, t13731: f64, t13734: f64, t193: f64, t336: f64, t4700: f64, t11094: f64, t1637: f64, t14257: f64, t14262: f64, t14376: f64, t14378: f64, t14381: f64, t14384: f64, t14387: f64, t14391: f64, t14394: f64, t14398: f64, t14424: f64, t14472: f64, t14475: f64, t14477: f64, t14479: f64, t14482: f64, t14484: f64, t14486: f64, t3209: f64, t3213: f64, t4701: f64, t25: f64, t265: f64, t394: f64, t13493: f64, t1074: f64, t12606: f64, t13503: f64, t13504: f64, t13506: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t4705: f64, t606: f64, t607: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t3640: f64, t5091: f64, t3415: f64, t4869: f64, t1654: f64, t2394: f64, t4734: f64, t690: f64, t1089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14657 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1855(t1003, t1058, t1061, t1063, t11037, t11046, t13940, t14615, t14618, t14623, t14627, t14631, t14640, t14645, t14648, t14651, t14654, t1610, t3180, t3186, t3189, t3197, t3200, t3204, t353, t384, t4615, t4669, t4685, t4689, t4691);
        let (t14658, t14659, t14661) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1856(t14613, t14657, t1055, t10160, t10170, t1052, t1066, t11010, t14545, t14549, t14552, t14555, t14562, t1635, t3169, t3176, t3207, t388, t4557, t4660, t4665);
        let (t14662, t14666) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1857(t14543, t14661, t1068, t1070, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13666, t13720, t13722, t13726, t13729, t13731, t13734, t193, t336, t4700);
        let (t14667, t14673) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1858(t11094, t1637, t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479, t14482, t14484, t14486, t3209, t3213, t4700, t4701);
        let (t14675, t14687) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1859(t25, t265, t394, t13493, t14666, t14673, t1074, t12606, t13503, t13504, t13506, t1408, t1409, t1534, t1642, t2249, t2250, t2756, t3220, t396, t3966, t40, t4324, t4705, t606, t607, dens_threshold, rho0, zeta_threshold);
        let (t14696, t14701, t14702) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1860(t3640, t5091, t3415, t4869, t1654, t2394);
        let t14704 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1861(t4734, t690);
        let (t14705, t14706) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1862(t14704, t1089, t12606);
    (t14658, t14659, t14662, t14667, t14675, t14687, t14696, t14701, t14702, t14704, t14705, t14706)
}
