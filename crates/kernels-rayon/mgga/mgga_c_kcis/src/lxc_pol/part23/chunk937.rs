//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 937/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk937(t17357: f64, t6027: f64, t12505: f64, t2039: f64, t2062: f64, t4278: f64, t12568: f64, t5919: f64, t12530: f64, t5916: f64, t2051: f64, t4303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17358 = t6027 * t17357;
    let t17360 = t12505 * t2039;
    let t17362 = t4278 * t2062;
    let t17364 = t12568 * t5919;
    let t17366 = t12530 * t5916;
    let t17368 = t2051 * t4303;
    (t17358, t17360, t17362, t17364, t17366, t17368)
}
