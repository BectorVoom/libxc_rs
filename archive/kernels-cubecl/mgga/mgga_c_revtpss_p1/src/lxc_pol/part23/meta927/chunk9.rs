//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3018/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3018<F: Float>(t3154: F, t6299: F, t12050: F, t12122: F, t16432: F, t16566: F, t16584: F, t19414: F, t19491: F, t19534: F, t19548: F, t19584: F, t19594: F, t19597: F, t24116: F, t24162: F, t3204: F, t3278: F, t342: F, t380: F, t43520: F, t43524: F, t4954: F, t5004: F, t55732: F, t55958: F, t56049: F, t6375: F, t65144: F, t79703: F, t80132: F, t80248: F, t989: F, t999: F) -> (F, F) {
    let t80277 = t3154 * t6299;
    let t80310 = F::cast_from(0.39512695097613069591e1_f64) * t4954 * t19584 - F::cast_from(0.39512695097613069591e1_f64) * t12122 * t16432 * t80277 * t999 + F::cast_from(0.19756347548806534796e1_f64) * t16566 * t65144 * t12050 * t79703 - F::cast_from(0.79025390195226139182e1_f64) * t56049 * t19594 + F::cast_from(0.39512695097613069591e1_f64) * t55732 * t19597 - F::cast_from(0.19756347548806534796e1_f64) * t16584 * t19534 + F::cast_from(0.65854491829355115987e0_f64) * t989 * t24162 + F::cast_from(0.19756347548806534796e1_f64) * t3278 * t24116 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t380 * t80132 + F::cast_from(0.39512695097613069592e1_f64) * t3204 * t5004 * t19414 + F::cast_from(0.39512695097613069591e1_f64) * t55958 * t6375 - F::cast_from(0.11853808529283920877e2_f64) * t43520 * t80248 * t19548 + F::cast_from(0.11853808529283920877e2_f64) * t43524 * t80248 * t19491;
    (t80277, t80310)
}
