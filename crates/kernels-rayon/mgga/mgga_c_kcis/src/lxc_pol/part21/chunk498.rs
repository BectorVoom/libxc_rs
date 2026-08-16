//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 498/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk498(t1131: f64, t3190: f64, t1021: f64, t1092: f64, t1134: f64, t2825: f64, t1018: f64, t341: f64, t1017: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3191 = t1131 * t3190;
    let t3192 = t1021 * t3191;
    let t3193 = t1092 * t3192;
    let t3195 = t2825 * t1134;
    let t3196 = t1092 * t3195;
    let t3198 = t1018 * t341;
    let t3200 = t86 * t1017 * t3198;
    (t3191, t3192, t3193, t3195, t3196, t3198, t3200)
}
