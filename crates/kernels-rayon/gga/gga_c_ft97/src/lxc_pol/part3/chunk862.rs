//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 862/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk862(t1060: f64, t2185: f64, t3450: f64, t3455: f64, t3578: f64, t574: f64, t12664: f64, t3483: f64, t144: f64, t3478: f64, t4790: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17394 = t2185 * t1060 * t3450;
    let t17398 = t574 * t3578 * t3455;
    let t17401 = t12664 * t3483;
    let t17402 = t144 * t17401;
    let t17406 = t574 * t3578 * t3478;
    let t17409 = t4790 * t604;
    (t17394, t17398, t17401, t17402, t17406, t17409)
}
