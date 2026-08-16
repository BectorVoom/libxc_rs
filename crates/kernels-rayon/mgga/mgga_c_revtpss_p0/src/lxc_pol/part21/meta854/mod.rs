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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta854(t12050: f64, t17710: f64, t17191: f64, t3555: f64, t1269: f64, t13147: f64, t460: f64, t1209: f64, t21455: f64, t5219: f64, t5477: f64, t17288: f64, t3754: f64, t12621: f64, t12699: f64, t12748: f64, t12757: f64, t1280: f64, t13161: f64, t17192: f64, t17849: f64, t17949: f64, t3670: f64, t3755: f64, t3756: f64, t45738: f64, t5351: f64, t5457: f64, t5458: f64, t5474: f64, t56543: f64, t57275: f64, t57325: f64, t57465: f64, t58785: f64, t17600: f64, t3153: f64, t12722: f64, t12629: f64, t12706: f64, t12709: f64, t12714: f64, t12727: f64, t12751: f64, t12756: f64, t13118: f64, t16757: f64, t17188: f64, t17905: f64, t17955: f64, t17958: f64, t21500: f64, t21579: f64, t3769: f64, t3783: f64, t45666: f64, t45707: f64, t5436: f64, t5480: f64, t57696: f64, t45785: f64, t487: f64, t13043: f64, t43350: f64, t45832: f64, t5462: f64, t1204: f64, t12723: f64, t12753: f64, t17172: f64, t17175: f64, t17808: f64, t17861: f64, t17945: f64, t21452: f64, t3552: f64, t3603: f64, t3666: f64, t3746: f64, t3774: f64, t45852: f64, t45868: f64, t471: f64, t5459: f64, t5466: f64, t58921: f64, t1811: f64, t21451: f64, t12717: f64, t12769: f64, t12975: f64, t13127: f64, t13129: f64, t17887: f64, t17888: f64, t17909: f64, t17921: f64, t17941: f64, t44843: f64, t45700: f64, t5326: f64, t5452: f64, t56561: f64, t73: f64, t3566: f64, t16756: f64, t3302: f64, t12719: f64, t12966: f64, t13134: f64, t13142: f64, t13143: f64, t16695: f64, t16696: f64, t16697: f64, t16776: f64, t17853: f64, t17855: f64, t1822: f64, t45634: f64, t45718: f64, t45726: f64, t5465: f64, t56530: f64, t56555: f64, t57536: f64, t58760: f64, t3781: f64, t5216: f64, t45618: f64, t44535: f64, t45607: f64, t13045: f64, t1234: f64, t12744: f64, t1285: f64, t1287: f64, t12987: f64, t13107: f64, t13153: f64, t17183: f64, t1774: f64, t17846: f64, t17869: f64, t1794: f64, t3782: f64, t3784: f64, t45624: f64, t5487: f64, t56620: f64, t56766: f64, t57578: f64, t59476: f64, t1248: f64, t16771: f64, t16775: f64, t17454: f64, t17818: f64, t17864: f64, t17876: f64, t17880: f64, t354: f64, t45654: f64, t45683: f64, t45715: f64, t45796: f64, t45859: f64, t45863: f64, t56825: f64, t56830: f64, t58793: f64, t58798: f64, t58804: f64, t17948: f64, t13126: f64, t1770: f64, t13148: f64, t13149: f64, t13150: f64, t17633: f64, t17826: f64, t17879: f64, t17893: f64, t17944: f64, t17952: f64, t1818: f64, t21456: f64, t3787: f64, t44832: f64, t45385: f64, t45659: f64, t5481: f64, t58780: f64, t1281: f64, t16763: f64, t16768: f64, t17170: f64, t17178: f64, t17289: f64, t17829: f64, t17875: f64, t17951: f64, t3763: f64, t5478: f64, t5491: f64, t56376: f64, t59032: f64, t17852: f64, t1284: f64, t5412: f64, t17845: f64, t17306: f64, t12741: f64, t13112: f64, t17345: f64, t17821: f64, t17856: f64, t17883: f64, t17934: f64, t3584: f64, t44421: f64, t5443: f64, t59187: f64, t12646: f64, t12713: f64, t12732: f64, t16751: f64, t17837: f64, t17840: f64, t3588: f64, t45697: f64, t5332: f64, t5449: f64, t5463: f64, t5464: f64, t5494: f64, t57264: f64, t59096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59650, t59657, t59671, t59674, t59681, t59686) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3223(t12050, t17710, t17191, t3555, t1269, t13147, t460, t1209, t21455, t5219, t5477, t17288, t3754);
        let t59689 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3224(t12621, t12699, t12748, t12757, t1280, t13161, t17192, t17849, t17949, t3670, t3755, t3756, t45738, t5351, t5457, t5458, t5474, t56543, t57275, t57325, t57465, t58785, t59650, t59657, t59671, t59674, t59681, t59686);
        let (t59699, t59724) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3225(t17600, t3153, t12722, t5219, t12629, t12706, t12709, t12714, t12727, t12748, t12751, t12756, t13118, t16757, t17188, t17905, t17955, t17958, t21500, t21579, t3756, t3769, t3783, t45666, t45707, t5351, t5436, t5457, t5480, t57696);
        let t59762 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3226(t45785, t460, t487, t13043, t43350, t45832, t5219, t5462, t1204, t12714, t12723, t12753, t17172, t17175, t17808, t17861, t17905, t17945, t21452, t3552, t3603, t3666, t3746, t3774, t45707, t45852, t45868, t471, t5459, t5466, t58921);
        let (t59784, t59797) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3227(t13043, t1811, t1209, t21451, t1204, t12717, t12753, t12769, t1280, t12975, t13127, t13129, t16757, t17600, t17887, t17888, t17909, t17921, t17941, t3666, t3746, t44843, t45700, t5326, t5452, t5458, t5459, t5466, t56561, t73);
        let (t59824, t59833) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3228(t17191, t3566, t16756, t3302, t12719, t12751, t12756, t1280, t12966, t13134, t13142, t13143, t16695, t16696, t16697, t16776, t17853, t17855, t1822, t3670, t45634, t45718, t45726, t5326, t5465, t56530, t56555, t57536, t58760, t59699, t59784);
        let t59877 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3229(t3781, t5216, t45618, t460, t487, t43350, t44535, t45607, t13045, t1234, t12744, t1280, t1285, t1287, t12975, t12987, t13043, t13107, t13153, t17183, t1774, t17846, t17853, t17869, t1794, t3782, t3783, t3784, t45624, t5487, t56620, t56766, t57578, t58921, t59476, t59650);
        let t59916 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3230(t1248, t12717, t12751, t1287, t16695, t16771, t16775, t17454, t17818, t17864, t17876, t17880, t354, t45654, t45683, t45715, t45796, t45859, t45863, t5351, t56825, t56830, t58793, t58798, t58804, t59650, t59824);
        let t59951 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3231(t1204, t17948, t1269, t13126, t460, t13147, t1770, t12706, t12717, t13148, t13149, t13150, t17633, t17826, t17879, t17893, t17944, t17952, t1818, t21456, t3746, t3787, t44832, t45385, t45659, t5216, t5481, t58780, t59650, t59784);
        let t59983 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3232(t1234, t1269, t12699, t12709, t12723, t1280, t1281, t1285, t1287, t16756, t16763, t16768, t17170, t17178, t17188, t17289, t17829, t17875, t17880, t17949, t17951, t3666, t3746, t3763, t45852, t5478, t5491, t56376, t57536, t59032);
        let t60022 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3233(t1204, t17852, t1209, t1284, t5412, t17845, t17306, t3754, t1234, t1248, t12719, t12741, t1287, t13112, t17178, t17345, t17633, t17821, t17849, t17856, t17864, t17883, t17934, t3552, t3584, t3755, t3756, t44421, t45666, t5436, t5443, t5477, t5481, t59187);
        let t60058 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3234(t1774, t487, t12646, t12713, t12732, t1285, t1287, t12975, t13143, t13149, t16751, t16756, t17837, t17840, t17955, t3552, t3588, t3666, t45634, t45654, t45659, t45697, t45718, t5332, t5412, t5449, t5459, t5463, t5464, t5494, t57264, t59096);
    (t59689, t59724, t59762, t59797, t59833, t59877, t59916, t59951, t59983, t60022, t60058)
}
