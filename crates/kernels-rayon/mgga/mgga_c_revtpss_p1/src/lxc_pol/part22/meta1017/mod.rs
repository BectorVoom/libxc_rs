//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1017 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3516;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3517;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3518;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3519;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1017(t11773: f64, t4954: f64, t1011: f64, t6284: f64, t697: f64, t19900: f64, t3241: f64, t11883: f64, t12004: f64, t16223: f64, t19707: f64, t19917: f64, t42740: f64, t42745: f64, t42756: f64, t54198: f64, t54222: f64, t54259: f64, t54857: f64, t6285: f64, t6331: f64, t19477: f64, t3153: f64, t15926: f64, t15950: f64, t16012: f64, t16045: f64, t16089: f64, t19705: f64, t19809: f64, t3092: f64, t3117: f64, t42781: f64, t42785: f64, t4772: f64, t4873: f64, t4899: f64, t4900: f64, t4919: f64, t54261: f64, t54303: f64, t54306: f64, t63258: f64, t63283: f64, t63288: f64, t905: f64, t15905: f64, t56017: f64, t55899: f64, t11703: f64, t11859: f64, t15606: f64, t15609: f64, t15908: f64, t15910: f64, t16020: f64, t16025: f64, t16067: f64, t16084: f64, t16095: f64, t16096: f64, t18936: f64, t19450: f64, t19501: f64, t19572: f64, t19758: f64, t19954: f64, t42675: f64, t43044: f64, t4891: f64, t4902: f64, t53669: f64, t54314: f64, t54324: f64, t54570: f64, t55985: f64, t64891: f64, t15700: f64, t19992: f64, t53405: f64, t16226: f64, t19997: f64, t11710: f64, t19777: f64, t3091: f64, t19644: f64, t15596: f64, t15605: f64, t15611: f64, t15618: f64, t15688: f64, t1664: f64, t19722: f64, t42967: f64, t43043: f64, t4912: f64, t53800: f64, t53855: f64, t54289: f64, t54341: f64, t54348: f64, t54542: f64, t6268: f64, t4866: f64, t906: f64, t15689: f64, t15691: f64, t16052: f64, t19973: f64, t3162: f64, t42795: f64, t54387: f64, t54407: f64, t54414: f64, t54432: f64, t54435: f64, t54438: f64, t54440: f64, t54443: f64, t54446: f64, t54469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66542, t66558) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3516(t11773, t4954, t1011, t6284, t697, t19900, t3241, t11883, t12004, t16223, t19707, t19917, t42740, t42745, t42756, t54198, t54222, t54259, t54857, t6285, t6331);
        let (t66565, t66591) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3517(t19477, t3153, t1011, t15926, t15950, t16012, t16045, t16089, t19705, t19809, t3092, t3117, t3241, t42781, t42785, t4772, t4873, t4899, t4900, t4919, t54261, t54303, t54306, t63258, t63283, t63288, t905);
        let t66631 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3518(t15905, t56017, t55899, t11703, t11859, t15606, t15609, t15908, t15910, t16020, t16025, t16067, t16084, t16095, t16096, t18936, t19450, t19501, t19572, t19758, t19954, t3117, t3241, t42675, t43044, t4891, t4902, t53669, t54314, t54324, t54570, t55985, t64891);
        let t66662 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3519(t15700, t19992, t53405, t16226, t19997, t11710, t19777, t3091, t19644, t15596, t15605, t15611, t15618, t15688, t1664, t19707, t19722, t42967, t43043, t4912, t53800, t53855, t54289, t54341, t54348, t54542, t6268);
        let (t66667, t66682) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3520(t4866, t906, t15689, t15691, t16052, t19973, t3162, t42795, t54387, t54407, t54414, t54432, t54435, t54438, t54440, t54443, t54446, t54469);
    (t66542, t66558, t66565, t66591, t66631, t66662, t66667, t66682)
}
