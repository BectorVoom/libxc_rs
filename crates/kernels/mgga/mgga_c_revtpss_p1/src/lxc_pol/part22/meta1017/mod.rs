//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1017 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3516;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3517;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3518;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3519;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1017<F: Float>(t11773: F, t4954: F, t1011: F, t6284: F, t697: F, t19900: F, t3241: F, t11883: F, t12004: F, t16223: F, t19707: F, t19917: F, t42740: F, t42745: F, t42756: F, t54198: F, t54222: F, t54259: F, t54857: F, t6285: F, t6331: F, t19477: F, t3153: F, t15926: F, t15950: F, t16012: F, t16045: F, t16089: F, t19705: F, t19809: F, t3092: F, t3117: F, t42781: F, t42785: F, t4772: F, t4873: F, t4899: F, t4900: F, t4919: F, t54261: F, t54303: F, t54306: F, t63258: F, t63283: F, t63288: F, t905: F, t15905: F, t56017: F, t55899: F, t11703: F, t11859: F, t15606: F, t15609: F, t15908: F, t15910: F, t16020: F, t16025: F, t16067: F, t16084: F, t16095: F, t16096: F, t18936: F, t19450: F, t19501: F, t19572: F, t19758: F, t19954: F, t42675: F, t43044: F, t4891: F, t4902: F, t53669: F, t54314: F, t54324: F, t54570: F, t55985: F, t64891: F, t15700: F, t19992: F, t53405: F, t16226: F, t19997: F, t11710: F, t19777: F, t3091: F, t19644: F, t15596: F, t15605: F, t15611: F, t15618: F, t15688: F, t1664: F, t19722: F, t42967: F, t43043: F, t4912: F, t53800: F, t53855: F, t54289: F, t54341: F, t54348: F, t54542: F, t6268: F, t4866: F, t906: F, t15689: F, t15691: F, t16052: F, t19973: F, t3162: F, t42795: F, t54387: F, t54407: F, t54414: F, t54432: F, t54435: F, t54438: F, t54440: F, t54443: F, t54446: F, t54469: F) -> (F, F, F, F, F, F, F, F) {
        let (t66542, t66558) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3516::<F>(t11773, t4954, t1011, t6284, t697, t19900, t3241, t11883, t12004, t16223, t19707, t19917, t42740, t42745, t42756, t54198, t54222, t54259, t54857, t6285, t6331);
        let (t66565, t66591) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3517::<F>(t19477, t3153, t1011, t15926, t15950, t16012, t16045, t16089, t19705, t19809, t3092, t3117, t3241, t42781, t42785, t4772, t4873, t4899, t4900, t4919, t54261, t54303, t54306, t63258, t63283, t63288, t905);
        let t66631 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3518::<F>(t15905, t56017, t55899, t11703, t11859, t15606, t15609, t15908, t15910, t16020, t16025, t16067, t16084, t16095, t16096, t18936, t19450, t19501, t19572, t19758, t19954, t3117, t3241, t42675, t43044, t4891, t4902, t53669, t54314, t54324, t54570, t55985, t64891);
        let t66662 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3519::<F>(t15700, t19992, t53405, t16226, t19997, t11710, t19777, t3091, t19644, t15596, t15605, t15611, t15618, t15688, t1664, t19707, t19722, t42967, t43043, t4912, t53800, t53855, t54289, t54341, t54348, t54542, t6268);
        let (t66667, t66682) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3520::<F>(t4866, t906, t15689, t15691, t16052, t19973, t3162, t42795, t54387, t54407, t54414, t54432, t54435, t54438, t54440, t54443, t54446, t54469);
    (t66542, t66558, t66565, t66591, t66631, t66662, t66667, t66682)
}
