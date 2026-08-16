//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 640/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk640(t6966: f64, t6974: f64, t1338: f64, t2085: f64, t1352: f64, t553: f64, t7191: f64, t1332: f64, t1336: f64, t2089: f64, t544: f64, t6971: f64, t6980: f64, t6984: f64) -> (f64, f64, f64, f64) {
    let t7202 = 0.38381794893125283518e-1_f64 * t6966;
    let t7204 = 0.82246703342411321825e-2_f64 * t6974;
    let t7208 = t1338 * t2085;
    let t7209 = t7208 * t1352;
    let t7211 = t553 * t7191;
    let t7213 = -t7202 - 0.3289868133696452873e-1_f64 * t6971 - t7204 - 0.16449340668482264365e-1_f64 * t6980 + 0.16449340668482264365e-1_f64 * t6984 + t1332 * t2089 - t1336 * t7209 + t544 * t7211;
    (t7208, t7209, t7211, t7213)
}
