//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 855/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk855(t1134: f64, t13181: f64, t1092: f64, t1800: f64, t2850: f64, t3202: f64, t3200: f64, t1018: f64, t1747: f64, t1017: f64, t86: f64, t3214: f64) -> (f64, f64, f64, f64, f64) {
    let t13182 = t13181 * t1134;
    let t13183 = t1092 * t13182;
    let t13186 = t1800 * t2850;
    let t13187 = t3202 * t13186;
    let t13188 = t3200 * t13187;
    let t13190 = t1018 * t1747;
    let t13192 = t86 * t1017 * t13190;
    let t13193 = t13192 * t3214;
    (t13183, t13186, t13188, t13192, t13193)
}
