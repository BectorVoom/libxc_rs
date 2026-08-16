//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 626/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk626(t15605: f64, t82: f64, t72: f64, t15284: f64, t15288: f64, t15292: f64, t3207: f64, t534: f64, t3225: f64, t8368: f64, t22: f64, t2447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15606 = t82 * t15605;
    let t15607 = t72 * t15606;
    let t15609 = 0.68186654135613354325e-2_f64 * t15284;
    let t15610 = 0.68186654135613354325e-2_f64 * t15288;
    let t15611 = 0.20455996240684006296e-1_f64 * t15292;
    let t15612 = t534 * t3207;
    let t15613 = t72 * t15612;
    let t15614 = t8368 * t3225;
    let t15615 = 0.34093327067806677161e-2_f64 * t15614;
    let t15616 = t2447 * t22;
    (t15606, t15607, t15609, t15610, t15611, t15612, t15613, t15615, t15616)
}
