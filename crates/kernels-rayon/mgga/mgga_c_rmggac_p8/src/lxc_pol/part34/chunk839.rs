//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 839/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk839(t3154: f64, t9087: f64, t14371: f64, t15336: f64, t14027: f64, t15340: f64, t70554: f64, t1550: f64, t2060: f64, t41091: f64, t41006: f64, t903: f64) -> (f64, f64, f64, f64, f64) {
    let t75060 = t9087 * t3154;
    let t75062 = t14371 * t15336;
    let t75065 = t15340 * t70554 * t14027;
    let t75069 = 0.5987120850931904282e-1_f64 * t1550 * t2060 * t41091;
    let t75072 = 0.8980681276397856423e-1_f64 * t903 * t2060 * t41006;
    (t75060, t75062, t75065, t75069, t75072)
}
