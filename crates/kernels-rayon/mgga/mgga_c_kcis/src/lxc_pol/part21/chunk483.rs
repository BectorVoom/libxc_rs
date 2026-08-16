//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 483/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk483(t1014: f64, t1088: f64, t239: f64, t740: f64, t313: f64, t1031: f64, t331: f64, t1027: f64, t1046: f64, t1071: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3052 = t1014 * t1088;
    let t3054 = t740 * t239;
    let t3056 = 0.46853067927761790996e-2_f64 * t3054 * t313;
    let t3057 = t331 * t1031;
    let t3059 = t1027 * t1046;
    let t3061 = t89 * t1071;
    (t3052, t3054, t3056, t3057, t3059, t3061)
}
