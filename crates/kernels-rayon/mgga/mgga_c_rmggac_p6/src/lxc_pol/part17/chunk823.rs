//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 823/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk823(t40479: f64, t1982: f64, t7428: f64, t8688: f64, t1627: f64, t2064: f64, t3928: f64, t34884: f64, t8668: f64, t8831: f64, t8836: f64, t8843: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40480 = 0.19863479950205658386e-4_f64 * t40479;
    let t40505 = t8688 * t7428 * t1982;
    let t40506 = 0.19863479950205658386e-4_f64 * t40505;
    let t40516 = t3928 * t2064 * t1627;
    let t40558 = t34884 * t8668;
    let t40559 = 0.24829349937757072982e-4_f64 * t40558;
    let t40560 = t34884 * t8831;
    let t40561 = 0.74488049813271218946e-4_f64 * t40560;
    let t40562 = t34884 * t8836;
    let t40563 = 0.74488049813271218946e-4_f64 * t40562;
    let t40564 = t34884 * t8843;
    (t40480, t40506, t40516, t40559, t40561, t40563, t40564)
}
