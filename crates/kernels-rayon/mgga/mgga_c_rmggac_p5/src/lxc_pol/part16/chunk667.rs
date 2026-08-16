//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 667/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk667(t884: f64, t9624: f64, t5888: f64, t8041: f64, t1356: f64, t9531: f64, t2474: f64, t290: f64) -> (f64, f64, f64, f64, f64) {
    let t9625 = t884 * t9624;
    let t9627 = t8041 * t5888;
    let t9628 = t1356 * t9627;
    let t9637 = t1356 * t9531;
    let t9639 = t290 * t2474;
    (t9625, t9627, t9628, t9637, t9639)
}
