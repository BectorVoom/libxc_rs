//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1078 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3860;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3861;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3862;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3863;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3864;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3865;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3866;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3867;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3868;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1078<F: Float>(t543: F, t74077: F, t74165: F, t221: F, t22253: F, t4018: F, t4019: F, t1388: F, t1390: F, t1410: F, t3829: F, t6816: F, t74010: F, t74015: F, t74017: F, t74022: F, t74024: F, t74029: F, t74033: F, t74037: F, t828: F, t9942: F, t125: F, t21969: F, t1399: F, t6883: F, t9816: F, t9818: F, t13999: F, t22271: F, t48919: F, t6869: F, t13716: F, t13944: F, t1872: F, t22096: F, t3889: F, t3934: F, t3936: F, t3944: F, t48508: F, t48510: F, t48595: F, t5674: F, t6849: F, t800: F, t9748: F, t13847: F, t22016: F, t48731: F, t73731: F, t13804: F, t22046: F, t46416: F, t48514: F, t48516: F, t48518: F, t48527: F, t48529: F, t48531: F, t48536: F, t48540: F, t48544: F, t5673: F, t73856: F, t22298: F, t48100: F, t22129: F, t2713: F, t3964: F, t22079: F, t4057: F, t48548: F, t48553: F, t48557: F, t48563: F, t48565: F, t5671: F, t73847: F, t9840: F, t22169: F, t46691: F, t22173: F, t9744: F, t6856: F, t9779: F, t6880: F, t22062: F, t9775: F, t13845: F, t22145: F, t22068: F, t9765: F, t22052: F, t3989: F, t22118: F, t22274: F, t3924: F, t4012: F, t48798: F, t73345: F, t9955: F, t22022: F, t22061: F, t808: F, t9845: F, t13920: F, t4003: F, t22085: F, t9962: F, t22182: F, t47215: F, t46730: F, t46951: F, t48573: F, t48577: F, t48591: F, t48593: F, t22021: F, t9793: F, t9794: F, t13785: F, t46671: F, t46695: F, t46702: F, t46704: F, t46706: F, t46712: F, t48600: F, t48603: F, t48614: F, t5755: F, t73906: F, t73908: F, t6876: F, t9909: F, t22026: F, t46929: F, t22135: F, t1353: F, t1868: F, t22040: F, t46723: F, t46741: F, t46757: F, t48637: F, t48645: F, t48655: F, t6836: F, t46760: F, t46767: F, t46787: F, t46789: F, t48664: F, t48666: F, t48668: F, t48685: F, t48687: F, t48690: F, t48692: F, t46800: F, t46804: F, t46810: F, t46812: F, t46817: F, t46820: F, t46824: F, t48696: F, t48700: F, t48709: F, t48734: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74167, t74176) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3860::<F>(t543, t74077, t74165, t221, t22253, t4018, t4019, t1388, t1390, t1410, t3829, t6816, t74010, t74015, t74017, t74022, t74024, t74029, t74033, t74037, t828, t9942);
        let t74215 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3861::<F>(t125, t21969, t1399, t6883, t9816, t9818, t13999, t22271, t48919, t6869, t13716, t13944, t1872, t22096, t3889, t3934, t3936, t3944, t48508, t48510, t48595, t543, t5674, t6849, t800, t9748);
        let t74234 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3862::<F>(t13847, t22016, t48731, t73731, t13804, t22046, t46416, t48514, t48516, t48518, t48527, t48529, t48531, t48536, t48540, t48544, t5673);
        let t74266 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3863::<F>(t13847, t1399, t73856, t9816, t22298, t48100, t22129, t2713, t3964, t22046, t22079, t3829, t3934, t4057, t48548, t48553, t48557, t48563, t48565, t5671, t5673, t6883, t73847, t800, t9748, t9840);
        let (t74269, t74271, t74277, t74279, t74281, t74288) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3864::<F>(t22169, t46691, t22173, t9744, t6856, t9779, t6880, t22062, t9775, t13845, t22145, t48100);
        let t74298 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3865::<F>(t22068, t9765, t22052, t3989, t1399, t1410, t22118, t22274, t3924, t3934, t4012, t48798, t73345, t74269, t74271, t74277, t74279, t74281, t74288, t828, t9955);
        let (t74314, t74329) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3866::<F>(t22022, t9775, t22061, t808, t9845, t13920, t4003, t22085, t9962, t22182, t47215, t22046, t22079, t3829, t3936, t46730, t46951, t48573, t48577, t48591, t48593, t5671, t5673, t5674, t6849, t800, t9840);
        let t74347 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3867::<F>(t22021, t9793, t9794, t13785, t46671, t46695, t46702, t46704, t46706, t46712, t48600, t48603, t48614, t5755, t73906, t73908);
        let t74375 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3868::<F>(t6876, t9909, t22026, t46929, t808, t22135, t9744, t1353, t13716, t1410, t1868, t22040, t3889, t3944, t4012, t46723, t46741, t46757, t48637, t48645, t48655, t6836, t800, t828, t9942);
        let (t74390, t74397) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3869::<F>(t46760, t46767, t46787, t46789, t48664, t48666, t48668, t48685, t48687, t48690, t48692, t46800, t46804, t46810, t46812, t46817, t46820, t46824, t48696, t48700, t48709, t48734);
    (t74167, t74176, t74215, t74234, t74266, t74298, t74314, t74329, t74347, t74375, t74390, t74397)
}
