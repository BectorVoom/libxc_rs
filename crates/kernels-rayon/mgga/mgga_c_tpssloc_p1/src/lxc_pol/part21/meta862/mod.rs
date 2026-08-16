//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta862 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3129;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3130;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3131;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3132;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3133;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3134;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3135;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3136;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3137;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3138;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3139;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta862(t16558: f64, t3450: f64, t11588: f64, t6138: f64, t3447: f64, t3451: f64, t4904: f64, t52036: f64, t15313: f64, t15338: f64, t18523: f64, t3448: f64, t6144: f64, t15402: f64, t18237: f64, t15376: f64, t15395: f64, t15406: f64, t3449: f64, t4900: f64, t4908: f64, t63294: f64, t63311: f64, t63372: f64, t63378: f64, t1887: f64, t337: f64, t5416: f64, t51968: f64, t11575: f64, t15409: f64, t15412: f64, t18427: f64, t3452: f64, t52096: f64, t63315: f64, t63368: f64, t63390: f64, t63402: f64, t63406: f64, t63410: f64, t63420: f64, t3446: f64, t61064: f64, t1176: f64, t1714: f64, t1184: f64, t15293: f64, t15382: f64, t3439: f64, t44424: f64, t44439: f64, t52074: f64, t52076: f64, t52081: f64, t52084: f64, t52086: f64, t52089: f64, t52092: f64, t52109: f64, t4928: f64, t1174: f64, t135: f64, t18525: f64, t11583: f64, t17691: f64, t12652: f64, t4723: f64, t3428: f64, t6109: f64, t6146: f64, t698: f64, t15320: f64, t457: f64, t460: f64, t4733: f64, t4919: f64, t52122: f64, t52124: f64, t52170: f64, t7319: f64, t974: f64, t6140: f64, t63841: f64, t63843: f64, t63845: f64, t63886: f64, t63888: f64, t63891: f64, t63893: f64, t63896: f64, t63899: f64, t63903: f64, t63906: f64, t63909: f64, t50846: f64, t50848: f64, t50853: f64, t63911: f64, t63914: f64, t63918: f64, t63921: f64, t63924: f64, t63927: f64, t63930: f64, t63933: f64, t63936: f64, t63939: f64, t43855: f64, t43859: f64, t43861: f64, t43863: f64, t44466: f64, t50968: f64, t50970: f64, t50972: f64, t50978: f64, t64003: f64, t64006: f64, t64045: f64, t51039: f64, t51041: f64, t51043: f64, t51051: f64, t51053: f64, t64074: f64, t64076: f64, t64079: f64, t64082: f64, t64085: f64, t64087: f64, t64089: f64, t64092: f64, t18321: f64, t3435: f64, t15390: f64, t1653: f64, t24705: f64, t3472: f64, t3478: f64, t44478: f64, t52127: f64, t52135: f64, t52138: f64, t52161: f64, t52271: f64, t15281: f64, t18563: f64, t3432: f64, t11529: f64, t6130: f64, t15282: f64, t4889: f64, t18558: f64, t3431: f64, t14730: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64756, t64765, t64770, t64773, t64775) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3129(t16558, t3450, t11588, t6138, t3447, t3451, t4904, t52036, t15313, t15338, t18523, t3448);
        let t64786 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3130(t11588, t6144, t3447, t3451, t15402, t18237, t15376, t15395, t15406, t3449, t4900, t4908, t63294, t63311, t63372, t63378, t64756, t64765, t64770, t64773, t64775);
        let t64823 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3131(t1887, t337, t5416, t3447, t4904, t51968, t11575, t15376, t15409, t15412, t18427, t3452, t4900, t4908, t52096, t63315, t63368, t63390, t63402, t63406, t63410, t63420);
        let t64845 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3132(t3446, t61064, t1176, t1714, t1184, t15293, t15382, t3439, t44424, t44439, t52074, t52076, t52081, t52084, t52086, t52089, t52092, t52109);
        let (t64851, t64858, t64870, t64874, t64878, t64881) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3133(t4928, t1174, t135, t18525, t11583, t17691, t12652, t4723, t3428, t6109, t6146, t698);
        let t64883 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3134(t1174, t15293, t15320, t3447, t3449, t457, t460, t4733, t4908, t4919, t52122, t52124, t52170, t64851, t64858, t64870, t64874, t64878, t64881, t7319, t974);
        let (t64885, t64903) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3135(t1174, t6140, t698, t63841, t63843, t63845, t63886, t63888, t63891, t63893, t63896, t63899, t63903, t63906, t63909);
        let t64916 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3136(t50846, t50848, t50853, t63911, t63914, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939);
        let t64929 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3137(t43855, t43859, t43861, t43863, t44466, t50968, t50970, t50972, t50978, t64003, t64006, t64045);
        let t64943 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3138(t51039, t51041, t51043, t51051, t51053, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let t64966 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3139(t18321, t3435, t1174, t15390, t1653, t24705, t3447, t3472, t3478, t44478, t457, t460, t4919, t52127, t52135, t52138, t52161, t52271, t64885, t64903, t64916, t64929, t64943, t974);
        let (t64969, t64976, t64979, t64981, t64988, t64990) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3140(t1174, t15281, t18563, t18321, t3432, t11529, t6130, t15282, t4889, t18558, t3431, t12652, t14730);
    (t64786, t64823, t64845, t64874, t64883, t64966, t64969, t64976, t64979, t64981, t64988, t64990)
}
