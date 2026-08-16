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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3213;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3214;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3215;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3216;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3217;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3218;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3219;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta958(t5412: f64, t6628: f64, t1287: f64, t12966: f64, t12987: f64, t17307: f64, t17888: f64, t17955: f64, t20721: f64, t20747: f64, t21430: f64, t21554: f64, t21583: f64, t24713: f64, t24941: f64, t24978: f64, t25002: f64, t3670: f64, t3759: f64, t3767: f64, t3769: f64, t45666: f64, t45715: f64, t5326: f64, t5486: f64, t59492: f64, t6727: f64, t72686: f64, t82881: f64, t24864: f64, t473: f64, t11249: f64, t24834: f64, t3153: f64, t1214: f64, t1234: f64, t17183: f64, t17846: f64, t17847: f64, t20956: f64, t21416: f64, t21439: f64, t21465: f64, t21468: f64, t21472: f64, t21500: f64, t21541: f64, t21562: f64, t21579: f64, t21582: f64, t21586: f64, t21596: f64, t45654: f64, t45659: f64, t45863: f64, t5230: f64, t5284: f64, t5436: f64, t5470: f64, t1269: f64, t24543: f64, t24704: f64, t24751: f64, t1248: f64, t12717: f64, t12751: f64, t12756: f64, t1285: f64, t13142: f64, t13143: f64, t21495: f64, t21535: f64, t24998: f64, t5458: f64, t5465: f64, t5478: f64, t5480: f64, t59730: f64, t70311: f64, t72329: f64, t72724: f64, t73: f64, t82775: f64, t82859: f64, t82886: f64, t1291: f64, t17821: f64, t21507: f64, t21542: f64, t21587: f64, t24698: f64, t3766: f64, t3781: f64, t45683: f64, t45738: f64, t45740: f64, t460: f64, t490: f64, t5466: f64, t5481: f64, t6587: f64, t6695: f64, t72343: f64, t72732: f64, t82293: f64, t82321: f64, t83232: f64, t17192: f64, t17861: f64, t17934: f64, t21427: f64, t21443: f64, t21452: f64, t21484: f64, t21491: f64, t21513: f64, t21518: f64, t21521: f64, t24919: f64, t3746: f64, t3755: f64, t5459: f64, t5463: f64, t57465: f64, t59681: f64, t59749: f64, t59788: f64, t60019: f64, t6735: f64, t72267: f64, t83330: f64, t12702: f64, t1280: f64, t17289: f64, t17958: f64, t21436: f64, t21480: f64, t21558: f64, t24616: f64, t24770: f64, t24974: f64, t44843: f64, t5216: f64, t5245: f64, t6573: f64, t6723: f64, t6741: f64, t82514: f64, t83551: f64, t13045: f64, t6622: f64, t1204: f64, t13148: f64, t13149: f64, t20900: f64, t21456: f64, t21459: f64, t21473: f64, t24915: f64, t24981: f64, t43350: f64, t45707: f64, t45852: f64, t471: f64, t5332: f64, t5446: f64, t5464: f64, t59650: f64, t59657: f64, t59737: f64, t6717: f64, t72270: f64, t72386: f64, t3603: f64, t17853: f64, t21442: f64, t21506: f64, t21512: f64, t21538: f64, t21557: f64, t21610: f64, t24739: f64, t24922: f64, t24928: f64, t6720: f64, t72143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t84415, t84425) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3213(t5412, t6628, t1287, t12966, t12987, t17307, t17888, t17955, t20721, t20747, t21430, t21554, t21583, t24713, t24941, t24978, t25002, t3670, t3759, t3767, t3769, t45666, t45715, t5326, t5486, t59492, t6727, t72686, t82881);
        let (t84450, t84457, t84461) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3214(t24864, t473, t11249, t24834, t3153, t1214, t1234, t17183, t17846, t17847, t20956, t21416, t21439, t21465, t21468, t21472, t21500, t21541, t21562, t21579, t21582, t21586, t21596, t3670, t45654, t45659, t45863, t5230, t5284, t5436, t5470);
        let (t84462, t84487, t84506) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3215(t1269, t24543, t24704, t3153, t24751, t1248, t12717, t12751, t12756, t1285, t1287, t13142, t13143, t21495, t21535, t24864, t24998, t5436, t5458, t5465, t5478, t5480, t59730, t70311, t72329, t72724, t73, t82775, t82859, t82886);
        let t84541 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3216(t1234, t12751, t1291, t17821, t21507, t21542, t21587, t24698, t25002, t3766, t3769, t3781, t45683, t45738, t45740, t460, t490, t5326, t5465, t5466, t5481, t6587, t6695, t72343, t72732, t82293, t82321, t83232, t84487);
        let t84570 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3217(t1287, t17192, t17861, t17934, t21427, t21443, t21452, t21484, t21491, t21513, t21518, t21521, t21596, t24919, t3746, t3755, t5459, t5463, t5465, t57465, t59681, t59749, t59788, t60019, t6735, t72267, t82859, t83330);
        let t84605 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3218(t1234, t1269, t12702, t1280, t1285, t1287, t12987, t17289, t17821, t17958, t21436, t21452, t21465, t21480, t21541, t21558, t21579, t24616, t24770, t24974, t3670, t3759, t44843, t5216, t5245, t5436, t6573, t6723, t6741, t82514, t83551);
        let t84641 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3219(t13045, t6622, t1204, t1248, t13148, t13149, t17192, t17846, t20900, t21456, t21459, t21468, t21473, t24915, t24981, t43350, t45707, t45852, t471, t5332, t5446, t5463, t5464, t59650, t59657, t59681, t59737, t6717, t72270, t72386, t82886, t84462);
        let (t84645, t84679) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3220(t3603, t6622, t1214, t1248, t12717, t12756, t12966, t17289, t17847, t17853, t21442, t21456, t21506, t21512, t21538, t21557, t21558, t21610, t24739, t24922, t24928, t3746, t45654, t45666, t45738, t5326, t5458, t59650, t6720, t72143, t73, t82293, t84450);
    (t84415, t84425, t84457, t84461, t84462, t84506, t84541, t84570, t84605, t84641, t84645, t84679)
}
