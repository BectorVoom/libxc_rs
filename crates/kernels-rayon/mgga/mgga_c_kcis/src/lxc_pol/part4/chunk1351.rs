//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1351/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1351(t17388: f64, t6010: f64, t4291: f64, t5747: f64, t4294: f64, t2066: f64, t4278: f64, t2033: f64, t4121: f64, t4257: f64, t12530: f64, t5913: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t17389 = t6010 * t17388;
    let t17391 = t5747 * t4291;
    let t17392 = t17391 * t4294;
    let t17394 = t4278 * t2066;
    let t17396 = t2033 * t4121;
    let t17397 = t17396 * sigma2;
    let t17398 = t17397 * t4257;
    let t17400 = t12530 * t5913;
    (t17389, t17392, t17394, t17398, t17400)
}
