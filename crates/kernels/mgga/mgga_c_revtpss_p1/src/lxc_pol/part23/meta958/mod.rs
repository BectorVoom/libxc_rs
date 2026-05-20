//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta958 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3213;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3214;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3215;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3216;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3217;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3218;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3219;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta958<F: Float>(t5412: F, t6628: F, t1287: F, t12966: F, t12987: F, t17307: F, t17888: F, t17955: F, t20721: F, t20747: F, t21430: F, t21554: F, t21583: F, t24713: F, t24941: F, t24978: F, t25002: F, t3670: F, t3759: F, t3767: F, t3769: F, t45666: F, t45715: F, t5326: F, t5486: F, t59492: F, t6727: F, t72686: F, t82881: F, t24864: F, t473: F, t11249: F, t24834: F, t3153: F, t1214: F, t1234: F, t17183: F, t17846: F, t17847: F, t20956: F, t21416: F, t21439: F, t21465: F, t21468: F, t21472: F, t21500: F, t21541: F, t21562: F, t21579: F, t21582: F, t21586: F, t21596: F, t45654: F, t45659: F, t45863: F, t5230: F, t5284: F, t5436: F, t5470: F, t1269: F, t24543: F, t24704: F, t24751: F, t1248: F, t12717: F, t12751: F, t12756: F, t1285: F, t13142: F, t13143: F, t21495: F, t21535: F, t24998: F, t5458: F, t5465: F, t5478: F, t5480: F, t59730: F, t70311: F, t72329: F, t72724: F, t73: F, t82775: F, t82859: F, t82886: F, t1291: F, t17821: F, t21507: F, t21542: F, t21587: F, t24698: F, t3766: F, t3781: F, t45683: F, t45738: F, t45740: F, t460: F, t490: F, t5466: F, t5481: F, t6587: F, t6695: F, t72343: F, t72732: F, t82293: F, t82321: F, t83232: F, t17192: F, t17861: F, t17934: F, t21427: F, t21443: F, t21452: F, t21484: F, t21491: F, t21513: F, t21518: F, t21521: F, t24919: F, t3746: F, t3755: F, t5459: F, t5463: F, t57465: F, t59681: F, t59749: F, t59788: F, t60019: F, t6735: F, t72267: F, t83330: F, t12702: F, t1280: F, t17289: F, t17958: F, t21436: F, t21480: F, t21558: F, t24616: F, t24770: F, t24974: F, t44843: F, t5216: F, t5245: F, t6573: F, t6723: F, t6741: F, t82514: F, t83551: F, t13045: F, t6622: F, t1204: F, t13148: F, t13149: F, t20900: F, t21456: F, t21459: F, t21473: F, t24915: F, t24981: F, t43350: F, t45707: F, t45852: F, t471: F, t5332: F, t5446: F, t5464: F, t59650: F, t59657: F, t59737: F, t6717: F, t72270: F, t72386: F, t3603: F, t17853: F, t21442: F, t21506: F, t21512: F, t21538: F, t21557: F, t21610: F, t24739: F, t24922: F, t24928: F, t6720: F, t72143: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t84415, t84425) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3213::<F>(t5412, t6628, t1287, t12966, t12987, t17307, t17888, t17955, t20721, t20747, t21430, t21554, t21583, t24713, t24941, t24978, t25002, t3670, t3759, t3767, t3769, t45666, t45715, t5326, t5486, t59492, t6727, t72686, t82881);
        let (t84450, t84457, t84461) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3214::<F>(t24864, t473, t11249, t24834, t3153, t1214, t1234, t17183, t17846, t17847, t20956, t21416, t21439, t21465, t21468, t21472, t21500, t21541, t21562, t21579, t21582, t21586, t21596, t3670, t45654, t45659, t45863, t5230, t5284, t5436, t5470);
        let (t84462, t84487, t84506) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3215::<F>(t1269, t24543, t24704, t3153, t24751, t1248, t12717, t12751, t12756, t1285, t1287, t13142, t13143, t21495, t21535, t24864, t24998, t5436, t5458, t5465, t5478, t5480, t59730, t70311, t72329, t72724, t73, t82775, t82859, t82886);
        let t84541 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3216::<F>(t1234, t12751, t1291, t17821, t21507, t21542, t21587, t24698, t25002, t3766, t3769, t3781, t45683, t45738, t45740, t460, t490, t5326, t5465, t5466, t5481, t6587, t6695, t72343, t72732, t82293, t82321, t83232, t84487);
        let t84570 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3217::<F>(t1287, t17192, t17861, t17934, t21427, t21443, t21452, t21484, t21491, t21513, t21518, t21521, t21596, t24919, t3746, t3755, t5459, t5463, t5465, t57465, t59681, t59749, t59788, t60019, t6735, t72267, t82859, t83330);
        let t84605 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3218::<F>(t1234, t1269, t12702, t1280, t1285, t1287, t12987, t17289, t17821, t17958, t21436, t21452, t21465, t21480, t21541, t21558, t21579, t24616, t24770, t24974, t3670, t3759, t44843, t5216, t5245, t5436, t6573, t6723, t6741, t82514, t83551);
        let t84641 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3219::<F>(t13045, t6622, t1204, t1248, t13148, t13149, t17192, t17846, t20900, t21456, t21459, t21468, t21473, t24915, t24981, t43350, t45707, t45852, t471, t5332, t5446, t5463, t5464, t59650, t59657, t59681, t59737, t6717, t72270, t72386, t82886, t84462);
        let (t84645, t84679) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3220::<F>(t3603, t6622, t1214, t1248, t12717, t12756, t12966, t17289, t17847, t17853, t21442, t21456, t21506, t21512, t21538, t21557, t21558, t21610, t24739, t24922, t24928, t3746, t45654, t45666, t45738, t5326, t5458, t59650, t6720, t72143, t73, t82293, t84450);
    (t84415, t84425, t84457, t84461, t84462, t84506, t84541, t84570, t84605, t84641, t84645, t84679)
}
