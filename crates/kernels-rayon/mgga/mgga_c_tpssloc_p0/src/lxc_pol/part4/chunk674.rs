//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 674/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk674(t343: f64, t4547: f64, t4546: f64, t1593: f64, t1600: f64, t2958: f64, t2960: f64, t2969: f64, t2972: f64, t2975: f64, t2986: f64, t4507: f64, t4511: f64, t4515: f64, t4519: f64, t4523: f64, t4529: f64, t4532: f64, t4543: f64, t973: f64) -> (f64, f64) {
    let t4548 = t4547 * t343;
    let t4549 = t4546 * t4548;
    let t4552 = -0.74074074074074074072e-3_f64 * t2958 - t2969 + 0.9259259259259259259e-4_f64 * t2972 - 0.27777777777777777777e-3_f64 * t2975 - 0.74074074074074074072e-3_f64 * t2960 * t1593 + 0.9259259259259259259e-4_f64 * t4507 + 0.37037037037037037036e-3_f64 * t2986 * t4511 - 0.27777777777777777777e-3_f64 * t2986 * t4515 - 0.55555555555555555554e-3_f64 * t2986 * t4519 + 0.27777777777777777777e-3_f64 * t973 * t4523 + 0.22222222222222222222e-2_f64 * t2960 * t1600 - 0.27777777777777777777e-3_f64 * t4529 - 0.27777777777777777777e-3_f64 * t2986 * t4532 - 0.83333333333333333332e-3_f64 * t973 * t4543 - 0.83333333333333333332e-3_f64 * t973 * t4549;
    (t4548, t4552)
}
