//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 686/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk686(t1624: f64, t236: f64, t9188: f64, t3351: f64, t1627: f64, t511: f64, t3352: f64, t515: f64, t8377: f64, t2286: f64, t7720: f64, t495: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9189 = t236 * t1624;
    let t9190 = t9188 * t9189;
    let t9191 = t3351 * t9190;
    let t9193 = t511 * t1627;
    let t9194 = t3352 * t9193;
    let t9195 = t3351 * t9194;
    let t9197 = t515 * t8377;
    let t9198 = t3352 * t9197;
    let t9199 = t3351 * t9198;
    let t9202 = t7720 * t2286;
    let t9205 = t511 * t558 * t495;
    (t9190, t9191, t9194, t9195, t9198, t9199, t9202, t9205)
}
