//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3002/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3002(t1581: f64, t49541: f64, t60887: f64, t14473: f64, t4498: f64, t60332: f64, t942: f64, t951: f64, t959: f64, t10623: f64, t5808: f64, t17954: f64, t2907: f64) -> (f64, f64, f64, f64, f64) {
    let t62742 = 0.14035736694323150897e2_f64 * t49541 * t1581 * t60887;
    let t62744 = 0.69263436422725855034e2_f64 * t14473 * t4498;
    let t62748 = 0.5848223622634646207e0_f64 * t959 * t942 * t60332 * t951;
    let t62750 = 0.5848223622634646207e0_f64 * t10623 * t5808;
    let t62753 = 0.35089341735807877242e1_f64 * t959 * t17954 * t2907;
    (t62742, t62744, t62748, t62750, t62753)
}
