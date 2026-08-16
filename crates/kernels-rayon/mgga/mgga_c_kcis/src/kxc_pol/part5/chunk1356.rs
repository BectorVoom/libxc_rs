//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1356/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1356(t22348: f64, t6010: f64, t21876: f64, t6028: f64, t6027: f64, t22341: f64, t5904: f64, t4292: f64, t22212: f64, t556: f64, t572: f64, t1533: f64) -> (f64, f64, f64, f64) {
    let t22349 = t6010 * t22348;
    let t22351 = t6028 * t21876;
    let t22352 = t6027 * t22351;
    let t22354 = t5904 * t22341;
    let t22355 = t4292 * t22354;
    let t22357 = t556 * t22212;
    let t22358 = t572 * t22357;
    let t22359 = t1533 * t22358;
    (t22349, t22352, t22355, t22359)
}
