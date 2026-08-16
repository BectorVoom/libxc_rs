//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 290/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk290(t1173: f64, t420: f64, t1161: f64, t1165: f64, t1176: f64, t1175: f64, t306: f64) -> (f64, f64, f64) {
    let t1355 = t1173 * t420;
    let t1359 = 0.41275e-2_f64 * t1161;
    let t1361 = 0.1982e-1_f64 * t1176 - t1359 - 0.41275e-2_f64 * t1165;
    let t1364 = t1355 * t1175 / 4.0_f64 + t306 * t1361 / 2.0_f64;
    (t1355, t1361, t1364)
}
