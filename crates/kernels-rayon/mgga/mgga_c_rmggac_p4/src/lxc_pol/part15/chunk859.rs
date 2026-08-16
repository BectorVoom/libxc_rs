//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 859/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk859(t9214: f64, t9227: f64, t9232: f64, t9234: f64, t9236: f64, t9238: f64, t38382: f64, t38965: f64, t39122: f64, t39308: f64, t40332: f64, t40623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42351 = 0.17025839957319135759e-4_f64 * t9214;
    let t42355 = 0.4726e1_f64 * t9227;
    let t42356 = 0.4726e1_f64 * t9232;
    let t42357 = 0.4726e1_f64 * t9234;
    let t42358 = 0.85129199786595678796e-5_f64 * t9236;
    let t42359 = 0.11974241701863808564e0_f64 * t9238;
    let t42600 = 0.2927036860455597649e0_f64 * t38382;
    let t42793 = 0.66211599834018861287e-4_f64 * t38965;
    let t42856 = 0.66211599834018861287e-4_f64 * t39122;
    let t42913 = 0.66211599834018861287e-4_f64 * t39308;
    let t43375 = 0.58540737209111952978e0_f64 * t40332;
    let t43492 = 0.2927036860455597649e0_f64 * t40623;
    (t42351, t42355, t42356, t42357, t42358, t42359, t42600, t42793, t42856, t42913, t43375, t43492)
}
