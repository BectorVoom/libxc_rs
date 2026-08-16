//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1713/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1713(t6305: f64, t6343: f64, t378: f64, t88714: f64, t1678: f64, t23640: f64, t1082: f64, t1087: f64, t1089: f64, t12047: f64, t12052: f64, t12149: f64, t1668: f64, t1689: f64, t19446: f64, t24042: f64, t24108: f64, t3204: f64, t3299: f64, t3304: f64, t3317: f64, t3318: f64, t43154: f64, t4954: f64, t4975: f64, t6258: f64, t80243: f64, t88815: f64, t89312: f64, t89320: f64) -> (f64, f64, f64) {
    let t89471 = t6343 * t6305;
    let t89490 = t378 * t88714;
    let t89503 = t1678 * t23640;
    let t89507 = 0.79025390195226139183e1_f64 * t3299 * t89471 * t3304 + 0.15805078039045227836e2_f64 * t12149 * t19446 * t4975 * t6258 + 0.52683593463484092788e1_f64 * t3204 * t1082 * t88815 + 0.26341796731742046395e1_f64 * t80243 * t1689 + 0.39512695097613069591e1_f64 * t3204 * t1082 * t89312 + 0.15805078039045227836e2_f64 * t43154 * t1082 * t89320 - 0.19756347548806534796e1_f64 * t3317 * t89490 * t3318 + 0.26341796731742046395e1_f64 * t4954 * t24108 - 0.39512695097613069592e1_f64 * t3317 * t89471 * t3318 + 0.26341796731742046395e1_f64 * t1087 * t24042 * t1668 * t1089 + 0.26341796731742046395e1_f64 * t12047 * t89503 * t12052;
    (t89490, t89503, t89507)
}
