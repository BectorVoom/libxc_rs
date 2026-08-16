//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3091/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3091(t18502: f64, t699: f64, t18499: f64, t136: f64, t3297: f64, t63394: f64, t63386: f64, t63390: f64, t18509: f64, t18507: f64, t1113: f64, t63410: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t64074 = t699 * t18502;
    let t64076 = t699 * t18499;
    let t64079 = t136 * t3297 * t63394;
    let t64082 = t136 * t3297 * t63386;
    let t64085 = t136 * t3297 * t63390;
    let t64087 = t699 * t18509;
    let t64089 = t699 * t18507;
    let t64092 = t136 * t1113 * t63410;
    (t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092)
}
