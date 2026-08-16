//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 613/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk613(t5272: f64, t716: f64, t736: f64, t1871: f64, t1929: f64, t1937: f64, t1931: f64, t1941: f64, t5060: f64, t732: f64, t5063: f64, t719: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5273 = t5272 * t716;
    let t5274 = t5273 * sigma2;
    let t5275 = t5274 * t736;
    let t5277 = t1929 * t1871;
    let t5278 = t5277 * sigma2;
    let t5279 = t5278 * t1937;
    let t5281 = t1931 * t1941;
    let t5283 = t732 * t5060;
    let t5284 = t5283 * sigma2;
    let t5285 = t719 * t5063;
    (t5274, t5275, t5277, t5278, t5279, t5281, t5283, t5284, t5285)
}
