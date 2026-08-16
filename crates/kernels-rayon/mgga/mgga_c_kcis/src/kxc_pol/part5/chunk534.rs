//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 534/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk534(t2675: f64, t851: f64, t843: f64, t189: f64, t197: f64, t2665: f64, t673: f64, t88: f64, t2354: f64, t47: f64, t2355: f64, t680: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2676 = t2675 * t851;
    let t2679 = t843 * t843;
    let t2680 = 1.0_f64 / t2679;
    let t2681 = t189 * t2680;
    let t2682 = t197 * t197;
    let t2683 = 1.0_f64 / t2682;
    let t2684 = t2665 * t2683;
    let t2690 = t88 * t673;
    let t2694 = t47 * t2354;
    let t2695 = t2355 * t680;
    (t2676, t2679, t2680, t2681, t2682, t2683, t2684, t2690, t2694, t2695)
}
