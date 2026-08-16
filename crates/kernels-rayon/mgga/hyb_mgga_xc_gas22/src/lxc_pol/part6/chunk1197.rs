//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1197/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1197(t2713: f64, t2723: f64, t7441: f64, t7443: f64, t1057: f64, t7508: f64, t1052: f64, t2742: f64, t2757: f64, t7449: f64, t7453: f64, t2634: f64, t2646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22170 = 0.3103560775156404018e4_f64 * t7441 * t2723 * t7443 * t2713;
    let t22171 = t1057 * t7508;
    let t22173 = t1052 * t7508;
    let t22175 = t2757 * t2742;
    let t22179 = 0.57895126195293126241e3_f64 * t7449 * t7453 * t2713;
    let t22181 = 1.0_f64 / t2634 / t2646;
    (t22170, t22171, t22173, t22175, t22179, t22181)
}
