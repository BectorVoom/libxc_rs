//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1018/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1018(t1098: f64, t4672: f64, t1758: f64, t3251: f64, t313: f64, t4625: f64, t1762: f64, t1071: f64, t1109: f64, t10415: f64, t1670: f64, t127: f64, t2840: f64, t368: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14260 = 0.13140859333333333333e-2_f64 * t1098 * t4672;
    let t14272 = t3251 * t1758;
    let t14282 = t313 * t4625;
    let t14299 = t3251 * t1762;
    let t14301 = t1109 * t1071;
    let t14316 = t10415 * t1670;
    let t14321 = t127 * t368 * t2840;
    (t14260, t14272, t14282, t14299, t14301, t14316, t14321)
}
