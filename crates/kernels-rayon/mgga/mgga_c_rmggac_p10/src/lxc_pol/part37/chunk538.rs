//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 538/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk538(t3046: f64, t352: f64, t1326: f64, t14309: f64, t2123: f64, t36: f64, t2079: f64, t262: f64, t13996: f64, t305: f64, t14003: f64, t5148: f64) -> (f64, f64, f64, f64, f64) {
    let t14310 = t3046 * t352;
    let t14312 = t14309 * t1326 * t14310;
    let t14314 = t36 * t2123;
    let t14316 = t2079 * t262 * t14314;
    let t14319 = 0.2993560425465952141e-1_f64 * t305 * t13996;
    let t14324 = 0.5987120850931904282e-1_f64 * t5148 * t14003;
    (t14312, t14314, t14316, t14319, t14324)
}
