//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1718/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1718(t378: f64, t88694: f64, t1024: f64, t1087: f64, t1089: f64, t15670: f64, t16509: f64, t16544: f64, t23598: f64, t24079: f64, t24093: f64, t24112: f64, t24132: f64, t24135: f64, t24144: f64, t24147: f64, t43347: f64, t43352: f64, t43537: f64, t43538: f64, t5004: f64, t53877: f64, t55899: f64, t56017: f64, t6299: f64, t6343: f64, t6386: f64, t67790: f64) -> (f64, f64) {
    let t89647 = t378 * t88694;
    let t89663 = -0.79025390195226139183e1_f64 * t16544 * t24132 - 0.79025390195226139183e1_f64 * t16544 * t24135 - 0.39512695097613069592e1_f64 * t67790 * t6386 - 0.15805078039045227836e2_f64 * t53877 * t24147 + 0.39512695097613069592e1_f64 * t1087 * t6343 * t6299 * t1089 - 0.15805078039045227836e2_f64 * t55899 * t24079 - 0.23707617058567841754e2_f64 * t43537 * t89647 * t43538 + 0.15805078039045227836e2_f64 * t43347 * t89647 * t43352 - 0.26341796731742046395e1_f64 * t1024 * t5004 * t23598 + 0.15805078039045227836e2_f64 * t15670 * t24144 + 0.15805078039045227836e2_f64 * t16509 * t24112 + 0.15805078039045227836e2_f64 * t56017 * t24093;
    (t89647, t89663)
}
