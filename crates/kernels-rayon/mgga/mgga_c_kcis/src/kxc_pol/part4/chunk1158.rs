//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1158/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1158(t14686: f64, t3437: f64, t10753: f64, t5073: f64, t1166: f64, t5185: f64, t3460: f64, t5083: f64, t13265: f64, t3338: f64, t5046: f64, t10787: f64, t5091: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14687 = t3437 * t14686;
    let t14689 = t10753 * t5073;
    let t14691 = t1166 * t5185;
    let t14693 = t5083 * t3460;
    let t14695 = t3338 * t13265;
    let t14696 = t5046 * t14695;
    let t14698 = t10787 * t5091;
    (t14687, t14689, t14691, t14693, t14696, t14698)
}
