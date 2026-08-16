//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 535/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk535(t2323: f64, t2892: f64, t97: f64, t2453: f64, t2454: f64, t990: f64, t1248: f64, t1217: f64, t413: f64, t298: f64, t302: f64, t994: f64, rho1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2894 = t97 * t2323 * t2892;
    let t2895 = 6.0_f64 * t2894;
    let t2896 = 2.0_f64 * t2453;
    let t2897 = 8.0_f64 * t2454;
    let t2900 = t990 * t990;
    let t2901 = t1248 * t2900;
    let t2904 = t413 + t1217;
    let t2905 = t298 * t2904;
    let t2910 = 1.0_f64 / t302 / t994 / rho1;
    (t2895, t2896, t2897, t2900, t2901, t2904, t2905, t2910)
}
