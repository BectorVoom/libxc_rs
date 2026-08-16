//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 836/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk836(t1207: f64, t3549: f64, t9725: f64, t3005: f64, t956: f64, t3025: f64, t3034: f64, t110: f64, t1263: f64, t1251: f64, t25: f64, t3612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10936 = t1207 * t3549;
    let t10945 = 0.53272592592592592592e-1_f64 * t9725;
    let t10960 = t956 * t3005;
    let t10974 = t3025 * t3034;
    let t10989 = t110 * t1263;
    let t10990 = t1251 * t10989;
    let t10992 = t25 * t3612;
    (t10936, t10945, t10960, t10974, t10990, t10992)
}
