//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1061 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3780;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3781;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3782;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3783;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3784;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3785;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3786;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3787;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3788;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3789;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3790;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1061<F: Float>(t1770: F, t17845: F, t17852: F, t17948: F, t1204: F, t12709: F, t12717: F, t17821: F, t17849: F, t17856: F, t17875: F, t17876: F, t17888: F, t17944: F, t17952: F, t20800: F, t21040: F, t21298: F, t21451: F, t21465: F, t21491: F, t21521: F, t21579: F, t21583: F, t3670: F, t44421: F, t45385: F, t45700: F, t5230: F, t5458: F, t5459: F, t5466: F, t5478: F, t59686: F, t60013: F, t6714: F, t6717: F, t73: F, t12751: F, t12756: F, t1280: F, t1285: F, t1287: F, t16695: F, t16757: F, t17178: F, t17188: F, t17192: F, t17880: F, t17905: F, t17949: F, t20795: F, t20956: F, t21448: F, t21456: F, t21468: F, t21471: F, t21500: F, t3584: F, t3588: F, t3755: F, t3767: F, t3769: F, t45744: F, t5446: F, t59657: F, t60008: F, t60019: F, t6695: F, t68669: F, t70824: F, t70944: F, t72050: F, t72165: F, t21164: F, t3153: F, t12723: F, t12744: F, t12975: F, t12987: F, t16772: F, t17183: F, t17307: F, t17811: F, t17840: F, t17869: F, t17884: F, t17945: F, t17955: F, t21416: F, t21436: F, t21452: F, t21495: F, t21596: F, t3746: F, t5465: F, t59650: F, t59817: F, t6720: F, t70741: F, t71606: F, t12699: F, t16697: F, t17818: F, t20850: F, t21439: F, t21507: F, t21587: F, t3568: F, t3763: F, t3774: F, t3783: F, t45859: F, t5216: F, t5464: F, t5480: F, t5494: F, t59591: F, t59674: F, t59788: F, t59945: F, t6735: F, t70413: F, t70513: F, t71258: F, t71839: F, t71854: F, t1234: F, t1774: F, t17807: F, t17834: F, t17837: F, t17861: F, t17909: F, t1794: F, t21484: F, t21541: F, t44843: F, t45715: F, t45764: F, t45863: F, t5436: F, t5463: F, t5491: F, t59488: F, t59681: F, t59705: F, t59749: F, t6738: F, t69624: F, t70120: F, t70311: F, t71940: F, t21257: F, t12702: F, t16768: F, t17172: F, t17175: F, t17822: F, t17846: F, t17848: F, t17853: F, t17855: F, t17941: F, t21427: F, t21455: F, t21535: F, t21554: F, t3666: F, t45675: F, t45679: F, t5326: F, t5470: F, t5481: F, t70890: F, t3140: F, t3566: F, t13147: F, t1811: F, t460: F, t1243: F, t12966: F, t16751: F, t16763: F, t17289: F, t17826: F, t17921: F, t21430: F, t3596: F, t3781: F, t3782: F, t45710: F, t487: F, t5412: F, t5449: F, t5474: F, t6727: F, t6731: F, t70693: F, t72359: F, t3603: F, t43350: F, t13126: F, t1248: F, t12713: F, t17331: F, t1825: F, t20747: F, t21342: F, t21459: F, t21512: F, t3601: F, t3759: F, t3778: F, t45654: F, t45659: F, t45697: F, t59730: F, t70235: F, t71480: F, t72303: F, t5497: F, t17288: F, t488: F, t5219: F, t1211: F, t1215: F, t1274: F, t1277: F, t1295: F, t17973: F, t17974: F, t17988: F, t18018: F, t18084: F, t1829: F, t3567: F, t3569: F, t3737: F, t3790: F, t5231: F, t5251: F, t56384: F, t56416: F, t6744: F, t69652: F, t72140: F, t72187: F, t72231: F, t72276: F, t72315: F, t72358: F, t72404: F, t69636: F, t1812: F, t1209: F, t1210: F, t12666: F, t1294: F, t13182: F, t17968: F, t17975: F, t17999: F, t18108: F, t18114: F, t21082: F, t21390: F, t5220: F, t5417: F, t5422: F, t5423: F, t56294: F, t56310: F, t56314: F, t56315: F, t6580: F, t6702: F, t13181: F, t1214: F, t5428: F, t12633: F, t17967: F, t17986: F, t18005: F, t18043: F, t18047: F, t18059: F, t18070: F, t18073: F, t20704: F, t21348: F, t21366: F, t21382: F, t21389: F, t3572: F, t3732: F, t5429: F, t56486: F, t17306: F, t21333: F, t12603: F, t18054: F, t18097: F, t18103: F, t20753: F, t3739: F, t5237: F, t5498: F, t56503: F, t56508: F, t6587: F, t6745: F) -> (F, F, F, F) {
        let t72449 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3780::<F>(t1770, t17845, t17852, t17948, t1204, t12709, t12717, t17821, t17849, t17856, t17875, t17876, t17888, t17944, t17952, t20800, t21040, t21298, t21451, t21465, t21491, t21521, t21579, t21583, t3670, t44421, t45385, t45700, t5230, t5458, t5459, t5466, t5478, t59686, t60013, t6714, t6717, t73);
        let t72492 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3781::<F>(t12709, t12751, t12756, t1280, t1285, t1287, t16695, t16757, t17178, t17188, t17192, t17880, t17905, t17949, t20795, t20956, t21448, t21456, t21468, t21471, t21500, t3584, t3588, t3670, t3755, t3767, t3769, t45744, t5446, t5458, t59657, t60008, t60019, t6695, t68669, t70824, t70944, t72050, t72165);
        let (t72526, t72530) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3782::<F>(t21164, t3153, t12723, t12744, t12751, t1280, t12975, t12987, t16772, t17178, t17183, t17192, t17307, t17811, t17840, t17869, t17884, t17945, t17949, t17955, t21416, t21436, t21452, t21491, t21495, t21579, t21596, t3746, t5465, t59650, t59817, t6720, t70741, t71606);
        let t72572 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3783::<F>(t12699, t12751, t12756, t1280, t1287, t12987, t16697, t17818, t20795, t20850, t21439, t21507, t21587, t3568, t3670, t3755, t3763, t3769, t3774, t3783, t45859, t5216, t5464, t5480, t5494, t59591, t59674, t59788, t59945, t6735, t70413, t70513, t71258, t71839, t71854, t72526);
        let t72618 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3784::<F>(t1234, t1280, t1285, t1287, t1774, t17807, t17834, t17837, t17861, t17909, t1794, t20795, t21471, t21484, t21541, t3568, t3670, t3755, t3767, t3769, t44843, t45715, t45764, t45863, t5436, t5446, t5463, t5465, t5478, t5480, t5491, t59488, t59681, t59705, t59749, t6738, t69624, t70120, t70311, t71940);
        let t72659 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3785::<F>(t21257, t3153, t1204, t12702, t12751, t12756, t16768, t17172, t17175, t17822, t17846, t17848, t17853, t17855, t17861, t17941, t20956, t21427, t21455, t21535, t21554, t3666, t3746, t45675, t45679, t5326, t5436, t5465, t5470, t5480, t5481, t70890);
        let t72708 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3786::<F>(t3140, t3566, t13147, t1811, t460, t1243, t12699, t12751, t1287, t12966, t16697, t16751, t16763, t17289, t1774, t17818, t17826, t17849, t17861, t17921, t20795, t21430, t3584, t3596, t3755, t3781, t3782, t3783, t45710, t487, t5326, t5412, t5436, t5449, t5464, t5474, t5481, t6727, t6731, t70693, t72359);
        let t72757 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3787::<F>(t3603, t43350, t13126, t1811, t460, t1248, t12709, t12713, t12751, t12756, t1285, t1287, t12987, t17331, t17834, t17837, t17848, t17855, t17875, t17952, t1825, t20747, t21342, t21439, t21459, t21512, t3601, t3759, t3769, t3778, t3783, t45654, t45659, t45697, t45859, t45863, t5459, t59657, t59674, t59730, t59788, t6717, t70235, t71480, t72303);
        let t72797 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3788::<F>(t3566, t6695, t5497, t1811, t5216, t17288, t488, t5219, t1211, t1215, t1274, t1277, t1295, t17973, t17974, t17988, t18018, t18084, t1829, t3567, t3569, t3737, t3790, t5231, t5251, t56384, t56416, t6744, t69652, t70513, t72140, t72187, t72231, t72276, t72315, t72358, t72404, t72449, t72492, t72530, t72572, t72618, t72659, t72708, t72757);
        let t72832 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3789::<F>(t487, t69636, t1812, t3566, t1209, t1210, t12666, t1274, t1277, t1294, t13182, t17968, t17973, t17975, t17988, t17999, t18108, t18114, t21082, t21390, t3569, t3790, t5220, t5417, t5422, t5423, t56294, t56310, t56314, t56315, t6580, t6702);
        let t72865 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3790::<F>(t13181, t1774, t1214, t5428, t12633, t1277, t17967, t17973, t17986, t18005, t18043, t18047, t18059, t18070, t18073, t1829, t20704, t21348, t21366, t21382, t21389, t3567, t3568, t3572, t3732, t5220, t5251, t5429, t56486, t6744);
        let t72899 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3791::<F>(t17306, t1811, t1209, t21342, t21333, t487, t1210, t1215, t12603, t1277, t1295, t18054, t18059, t18097, t18103, t1829, t20753, t3567, t3568, t3569, t3737, t3739, t3790, t5237, t5498, t56503, t56508, t6587, t6702, t6745);
    (t72797, t72832, t72865, t72899)
}
