//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 856/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk856(t3957: f64, t3969: f64, t110: f64, t1121: f64, t3711: f64, t3960: f64, t410: f64, t959: f64, t968: f64, t3742: f64, t3966: f64, t3760: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8555 = t3969 * t3957;
    let t8559 = 3.8527786510141255_f64 * t1121 * t110 * t3711;
    let t8560 = t3969 * t3960;
    let t8564 = 0.04337432520120696_f64 * t1121 * t410 * t959;
    let t8567 = 1.2842595503380418_f64 * t1121 * t410 * t968;
    let t8570 = 38.025319932552506_f64 * t1121 * t110 * t3742;
    let t8576 = t3969 * t3966;
    let t8580 = 0.02168716260060348_f64 * t1121 * t110 * t3760;
    (t8555, t8559, t8560, t8564, t8567, t8570, t8576, t8580)
}
