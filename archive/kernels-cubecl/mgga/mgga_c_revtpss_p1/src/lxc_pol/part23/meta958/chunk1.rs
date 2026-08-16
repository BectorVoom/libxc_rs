//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3214/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3214<F: Float>(t24864: F, t473: F, t11249: F, t24834: F, t3153: F, t1214: F, t1234: F, t17183: F, t17846: F, t17847: F, t20956: F, t21416: F, t21439: F, t21465: F, t21468: F, t21472: F, t21500: F, t21541: F, t21562: F, t21579: F, t21582: F, t21586: F, t21596: F, t3670: F, t45654: F, t45659: F, t45863: F, t5230: F, t5284: F, t5436: F, t5470: F) -> (F, F, F) {
    let t84429 = t473 * t24864;
    let t84450 = t24834 * t11249;
    let t84457 = t24834 * t3153;
    let t84461 = -F::cast_from(0.19756347548806534796e1_f64) * t17183 * t21416 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t84429 * t1214 + F::cast_from(0.39512695097613069592e1_f64) * t3670 * t21541 * t5230 + F::cast_from(0.79025390195226139182e1_f64) * t21500 * t21596 + F::cast_from(0.19756347548806534796e1_f64) * t5436 * t21562 + F::cast_from(0.19756347548806534796e1_f64) * t21439 * t5470 + F::cast_from(0.39512695097613069592e1_f64) * t21500 * t21465 - F::cast_from(0.19756347548806534796e1_f64) * t21579 * t21468 + F::cast_from(0.11853808529283920877e2_f64) * t17846 * t20956 * t17847 * t5284 - F::cast_from(0.11853808529283920877e2_f64) * t45654 * t84450 * t21582 + F::cast_from(0.11853808529283920877e2_f64) * t45659 * t84450 * t21586 - F::cast_from(0.39512695097613069592e1_f64) * t45863 * t84457 * t21472;
    (t84450, t84457, t84461)
}
