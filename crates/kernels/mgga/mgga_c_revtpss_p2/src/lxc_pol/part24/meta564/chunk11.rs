//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1713/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1713<F: Float>(t6305: F, t6343: F, t378: F, t88714: F, t1678: F, t23640: F, t1082: F, t1087: F, t1089: F, t12047: F, t12052: F, t12149: F, t1668: F, t1689: F, t19446: F, t24042: F, t24108: F, t3204: F, t3299: F, t3304: F, t3317: F, t3318: F, t43154: F, t4954: F, t4975: F, t6258: F, t80243: F, t88815: F, t89312: F, t89320: F) -> (F, F, F) {
    let t89471 = t6343 * t6305;
    let t89490 = t378 * t88714;
    let t89503 = t1678 * t23640;
    let t89507 = F::cast_from(0.79025390195226139183e1_f64) * t3299 * t89471 * t3304 + F::cast_from(0.15805078039045227836e2_f64) * t12149 * t19446 * t4975 * t6258 + F::cast_from(0.52683593463484092788e1_f64) * t3204 * t1082 * t88815 + F::cast_from(0.26341796731742046395e1_f64) * t80243 * t1689 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t1082 * t89312 + F::cast_from(0.15805078039045227836e2_f64) * t43154 * t1082 * t89320 - F::cast_from(0.19756347548806534796e1_f64) * t3317 * t89490 * t3318 + F::cast_from(0.26341796731742046395e1_f64) * t4954 * t24108 - F::cast_from(0.39512695097613069592e1_f64) * t3317 * t89471 * t3318 + F::cast_from(0.26341796731742046395e1_f64) * t1087 * t24042 * t1668 * t1089 + F::cast_from(0.26341796731742046395e1_f64) * t12047 * t89503 * t12052;
    (t89490, t89503, t89507)
}
