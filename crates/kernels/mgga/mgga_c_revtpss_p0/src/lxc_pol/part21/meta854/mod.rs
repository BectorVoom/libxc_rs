//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta854 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3223;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3224;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3225;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3226;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3227;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3228;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3229;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3230;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3231;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3232;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3233;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta854<F: Float>(t12050: F, t17710: F, t17191: F, t3555: F, t1269: F, t13147: F, t460: F, t1209: F, t21455: F, t5219: F, t5477: F, t17288: F, t3754: F, t12621: F, t12699: F, t12748: F, t12757: F, t1280: F, t13161: F, t17192: F, t17849: F, t17949: F, t3670: F, t3755: F, t3756: F, t45738: F, t5351: F, t5457: F, t5458: F, t5474: F, t56543: F, t57275: F, t57325: F, t57465: F, t58785: F, t17600: F, t3153: F, t12722: F, t12629: F, t12706: F, t12709: F, t12714: F, t12727: F, t12751: F, t12756: F, t13118: F, t16757: F, t17188: F, t17905: F, t17955: F, t17958: F, t21500: F, t21579: F, t3769: F, t3783: F, t45666: F, t45707: F, t5436: F, t5480: F, t57696: F, t45785: F, t487: F, t13043: F, t43350: F, t45832: F, t5462: F, t1204: F, t12723: F, t12753: F, t17172: F, t17175: F, t17808: F, t17861: F, t17945: F, t21452: F, t3552: F, t3603: F, t3666: F, t3746: F, t3774: F, t45852: F, t45868: F, t471: F, t5459: F, t5466: F, t58921: F, t1811: F, t21451: F, t12717: F, t12769: F, t12975: F, t13127: F, t13129: F, t17887: F, t17888: F, t17909: F, t17921: F, t17941: F, t44843: F, t45700: F, t5326: F, t5452: F, t56561: F, t73: F, t3566: F, t16756: F, t3302: F, t12719: F, t12966: F, t13134: F, t13142: F, t13143: F, t16695: F, t16696: F, t16697: F, t16776: F, t17853: F, t17855: F, t1822: F, t45634: F, t45718: F, t45726: F, t5465: F, t56530: F, t56555: F, t57536: F, t58760: F, t3781: F, t5216: F, t45618: F, t44535: F, t45607: F, t13045: F, t1234: F, t12744: F, t1285: F, t1287: F, t12987: F, t13107: F, t13153: F, t17183: F, t1774: F, t17846: F, t17869: F, t1794: F, t3782: F, t3784: F, t45624: F, t5487: F, t56620: F, t56766: F, t57578: F, t59476: F, t1248: F, t16771: F, t16775: F, t17454: F, t17818: F, t17864: F, t17876: F, t17880: F, t354: F, t45654: F, t45683: F, t45715: F, t45796: F, t45859: F, t45863: F, t56825: F, t56830: F, t58793: F, t58798: F, t58804: F, t17948: F, t13126: F, t1770: F, t13148: F, t13149: F, t13150: F, t17633: F, t17826: F, t17879: F, t17893: F, t17944: F, t17952: F, t1818: F, t21456: F, t3787: F, t44832: F, t45385: F, t45659: F, t5481: F, t58780: F, t1281: F, t16763: F, t16768: F, t17170: F, t17178: F, t17289: F, t17829: F, t17875: F, t17951: F, t3763: F, t5478: F, t5491: F, t56376: F, t59032: F, t17852: F, t1284: F, t5412: F, t17845: F, t17306: F, t12741: F, t13112: F, t17345: F, t17821: F, t17856: F, t17883: F, t17934: F, t3584: F, t44421: F, t5443: F, t59187: F, t12646: F, t12713: F, t12732: F, t16751: F, t17837: F, t17840: F, t3588: F, t45697: F, t5332: F, t5449: F, t5463: F, t5464: F, t5494: F, t57264: F, t59096: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59650, t59657, t59671, t59674, t59681, t59686) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3223::<F>(t12050, t17710, t17191, t3555, t1269, t13147, t460, t1209, t21455, t5219, t5477, t17288, t3754);
        let t59689 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3224::<F>(t12621, t12699, t12748, t12757, t1280, t13161, t17192, t17849, t17949, t3670, t3755, t3756, t45738, t5351, t5457, t5458, t5474, t56543, t57275, t57325, t57465, t58785, t59650, t59657, t59671, t59674, t59681, t59686);
        let (t59699, t59724) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3225::<F>(t17600, t3153, t12722, t5219, t12629, t12706, t12709, t12714, t12727, t12748, t12751, t12756, t13118, t16757, t17188, t17905, t17955, t17958, t21500, t21579, t3756, t3769, t3783, t45666, t45707, t5351, t5436, t5457, t5480, t57696);
        let t59762 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3226::<F>(t45785, t460, t487, t13043, t43350, t45832, t5219, t5462, t1204, t12714, t12723, t12753, t17172, t17175, t17808, t17861, t17905, t17945, t21452, t3552, t3603, t3666, t3746, t3774, t45707, t45852, t45868, t471, t5459, t5466, t58921);
        let (t59784, t59797) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3227::<F>(t13043, t1811, t1209, t21451, t1204, t12717, t12753, t12769, t1280, t12975, t13127, t13129, t16757, t17600, t17887, t17888, t17909, t17921, t17941, t3666, t3746, t44843, t45700, t5326, t5452, t5458, t5459, t5466, t56561, t73);
        let (t59824, t59833) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3228::<F>(t17191, t3566, t16756, t3302, t12719, t12751, t12756, t1280, t12966, t13134, t13142, t13143, t16695, t16696, t16697, t16776, t17853, t17855, t1822, t3670, t45634, t45718, t45726, t5326, t5465, t56530, t56555, t57536, t58760, t59699, t59784);
        let t59877 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3229::<F>(t3781, t5216, t45618, t460, t487, t43350, t44535, t45607, t13045, t1234, t12744, t1280, t1285, t1287, t12975, t12987, t13043, t13107, t13153, t17183, t1774, t17846, t17853, t17869, t1794, t3782, t3783, t3784, t45624, t5487, t56620, t56766, t57578, t58921, t59476, t59650);
        let t59916 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3230::<F>(t1248, t12717, t12751, t1287, t16695, t16771, t16775, t17454, t17818, t17864, t17876, t17880, t354, t45654, t45683, t45715, t45796, t45859, t45863, t5351, t56825, t56830, t58793, t58798, t58804, t59650, t59824);
        let t59951 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3231::<F>(t1204, t17948, t1269, t13126, t460, t13147, t1770, t12706, t12717, t13148, t13149, t13150, t17633, t17826, t17879, t17893, t17944, t17952, t1818, t21456, t3746, t3787, t44832, t45385, t45659, t5216, t5481, t58780, t59650, t59784);
        let t59983 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3232::<F>(t1234, t1269, t12699, t12709, t12723, t1280, t1281, t1285, t1287, t16756, t16763, t16768, t17170, t17178, t17188, t17289, t17829, t17875, t17880, t17949, t17951, t3666, t3746, t3763, t45852, t5478, t5491, t56376, t57536, t59032);
        let t60022 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3233::<F>(t1204, t17852, t1209, t1284, t5412, t17845, t17306, t3754, t1234, t1248, t12719, t12741, t1287, t13112, t17178, t17345, t17633, t17821, t17849, t17856, t17864, t17883, t17934, t3552, t3584, t3755, t3756, t44421, t45666, t5436, t5443, t5477, t5481, t59187);
        let t60058 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3234::<F>(t1774, t487, t12646, t12713, t12732, t1285, t1287, t12975, t13143, t13149, t16751, t16756, t17837, t17840, t17955, t3552, t3588, t3666, t45634, t45654, t45659, t45697, t45718, t5332, t5412, t5449, t5459, t5463, t5464, t5494, t57264, t59096);
    (t59689, t59724, t59762, t59797, t59833, t59877, t59916, t59951, t59983, t60022, t60058)
}
