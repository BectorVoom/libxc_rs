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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1078(t543: f64, t74077: f64, t74165: f64, t221: f64, t22253: f64, t4018: f64, t4019: f64, t1388: f64, t1390: f64, t1410: f64, t3829: f64, t6816: f64, t74010: f64, t74015: f64, t74017: f64, t74022: f64, t74024: f64, t74029: f64, t74033: f64, t74037: f64, t828: f64, t9942: f64, t125: f64, t21969: f64, t1399: f64, t6883: f64, t9816: f64, t9818: f64, t13999: f64, t22271: f64, t48919: f64, t6869: f64, t13716: f64, t13944: f64, t1872: f64, t22096: f64, t3889: f64, t3934: f64, t3936: f64, t3944: f64, t48508: f64, t48510: f64, t48595: f64, t5674: f64, t6849: f64, t800: f64, t9748: f64, t13847: f64, t22016: f64, t48731: f64, t73731: f64, t13804: f64, t22046: f64, t46416: f64, t48514: f64, t48516: f64, t48518: f64, t48527: f64, t48529: f64, t48531: f64, t48536: f64, t48540: f64, t48544: f64, t5673: f64, t73856: f64, t22298: f64, t48100: f64, t22129: f64, t2713: f64, t3964: f64, t22079: f64, t4057: f64, t48548: f64, t48553: f64, t48557: f64, t48563: f64, t48565: f64, t5671: f64, t73847: f64, t9840: f64, t22169: f64, t46691: f64, t22173: f64, t9744: f64, t6856: f64, t9779: f64, t6880: f64, t22062: f64, t9775: f64, t13845: f64, t22145: f64, t22068: f64, t9765: f64, t22052: f64, t3989: f64, t22118: f64, t22274: f64, t3924: f64, t4012: f64, t48798: f64, t73345: f64, t9955: f64, t22022: f64, t22061: f64, t808: f64, t9845: f64, t13920: f64, t4003: f64, t22085: f64, t9962: f64, t22182: f64, t47215: f64, t46730: f64, t46951: f64, t48573: f64, t48577: f64, t48591: f64, t48593: f64, t22021: f64, t9793: f64, t9794: f64, t13785: f64, t46671: f64, t46695: f64, t46702: f64, t46704: f64, t46706: f64, t46712: f64, t48600: f64, t48603: f64, t48614: f64, t5755: f64, t73906: f64, t73908: f64, t6876: f64, t9909: f64, t22026: f64, t46929: f64, t22135: f64, t1353: f64, t1868: f64, t22040: f64, t46723: f64, t46741: f64, t46757: f64, t48637: f64, t48645: f64, t48655: f64, t6836: f64, t46760: f64, t46767: f64, t46787: f64, t46789: f64, t48664: f64, t48666: f64, t48668: f64, t48685: f64, t48687: f64, t48690: f64, t48692: f64, t46800: f64, t46804: f64, t46810: f64, t46812: f64, t46817: f64, t46820: f64, t46824: f64, t48696: f64, t48700: f64, t48709: f64, t48734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74167, t74176) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3860(t543, t74077, t74165, t221, t22253, t4018, t4019, t1388, t1390, t1410, t3829, t6816, t74010, t74015, t74017, t74022, t74024, t74029, t74033, t74037, t828, t9942);
        let t74215 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3861(t125, t21969, t1399, t6883, t9816, t9818, t13999, t22271, t48919, t6869, t13716, t13944, t1872, t22096, t3889, t3934, t3936, t3944, t48508, t48510, t48595, t543, t5674, t6849, t800, t9748);
        let t74234 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3862(t13847, t22016, t48731, t73731, t13804, t22046, t46416, t48514, t48516, t48518, t48527, t48529, t48531, t48536, t48540, t48544, t5673);
        let t74266 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3863(t13847, t1399, t73856, t9816, t22298, t48100, t22129, t2713, t3964, t22046, t22079, t3829, t3934, t4057, t48548, t48553, t48557, t48563, t48565, t5671, t5673, t6883, t73847, t800, t9748, t9840);
        let (t74269, t74271, t74277, t74279, t74281, t74288) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3864(t22169, t46691, t22173, t9744, t6856, t9779, t6880, t22062, t9775, t13845, t22145, t48100);
        let t74298 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3865(t22068, t9765, t22052, t3989, t1399, t1410, t22118, t22274, t3924, t3934, t4012, t48798, t73345, t74269, t74271, t74277, t74279, t74281, t74288, t828, t9955);
        let (t74314, t74329) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3866(t22022, t9775, t22061, t808, t9845, t13920, t4003, t22085, t9962, t22182, t47215, t22046, t22079, t3829, t3936, t46730, t46951, t48573, t48577, t48591, t48593, t5671, t5673, t5674, t6849, t800, t9840);
        let t74347 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3867(t22021, t9793, t9794, t13785, t46671, t46695, t46702, t46704, t46706, t46712, t48600, t48603, t48614, t5755, t73906, t73908);
        let t74375 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3868(t6876, t9909, t22026, t46929, t808, t22135, t9744, t1353, t13716, t1410, t1868, t22040, t3889, t3944, t4012, t46723, t46741, t46757, t48637, t48645, t48655, t6836, t800, t828, t9942);
        let (t74390, t74397) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3869(t46760, t46767, t46787, t46789, t48664, t48666, t48668, t48685, t48687, t48690, t48692, t46800, t46804, t46810, t46812, t46817, t46820, t46824, t48696, t48700, t48709, t48734);
    (t74167, t74176, t74215, t74234, t74266, t74298, t74314, t74329, t74347, t74375, t74390, t74397)
}
