//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1718/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1718<F: Float>(t378: F, t88694: F, t1024: F, t1087: F, t1089: F, t15670: F, t16509: F, t16544: F, t23598: F, t24079: F, t24093: F, t24112: F, t24132: F, t24135: F, t24144: F, t24147: F, t43347: F, t43352: F, t43537: F, t43538: F, t5004: F, t53877: F, t55899: F, t56017: F, t6299: F, t6343: F, t6386: F, t67790: F) -> (F, F) {
    let t89647 = t378 * t88694;
    let t89663 = -F::cast_from(0.79025390195226139183e1_f64) * t16544 * t24132 - F::cast_from(0.79025390195226139183e1_f64) * t16544 * t24135 - F::cast_from(0.39512695097613069592e1_f64) * t67790 * t6386 - F::cast_from(0.15805078039045227836e2_f64) * t53877 * t24147 + F::cast_from(0.39512695097613069592e1_f64) * t1087 * t6343 * t6299 * t1089 - F::cast_from(0.15805078039045227836e2_f64) * t55899 * t24079 - F::cast_from(0.23707617058567841754e2_f64) * t43537 * t89647 * t43538 + F::cast_from(0.15805078039045227836e2_f64) * t43347 * t89647 * t43352 - F::cast_from(0.26341796731742046395e1_f64) * t1024 * t5004 * t23598 + F::cast_from(0.15805078039045227836e2_f64) * t15670 * t24144 + F::cast_from(0.15805078039045227836e2_f64) * t16509 * t24112 + F::cast_from(0.15805078039045227836e2_f64) * t56017 * t24093;
    (t89647, t89663)
}
