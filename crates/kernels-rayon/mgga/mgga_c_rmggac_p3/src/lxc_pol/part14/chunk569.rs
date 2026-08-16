//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 569/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk569(t2147: f64, t7508: f64, t649: f64, t866: f64, t27: f64, t2145: f64, t645: f64, t798: f64, t3928: f64, t2060: f64, t4048: f64, t1550: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7509 = t7508 * t2147;
    let t7510 = 0.68186654135613354322e-2_f64 * t7509;
    let t7511 = t649 * t866;
    let t7512 = t27 * t7511;
    let t7513 = t2145 * t7512;
    let t7514 = 0.34093327067806677161e-2_f64 * t7513;
    let t7518 = t645 * t798;
    let t7519 = t3928 * t7518;
    let t7520 = 0.17961362552795712846e0_f64 * t7519;
    let t7521 = t2060 * t4048;
    let t7522 = t1550 * t7521;
    (t7510, t7512, t7514, t7518, t7520, t7521, t7522)
}
