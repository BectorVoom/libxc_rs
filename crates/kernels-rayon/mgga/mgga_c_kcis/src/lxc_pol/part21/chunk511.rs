//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 511/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk511(t3245: f64, t361: f64, t1014: f64, t1127: f64, t126: f64, t88: f64, t85: f64) -> (f64, f64, f64, f64, f64) {
    let t3246 = t3245 * t361;
    let t3247 = 0.55273148148148148147e-3_f64 * t3246;
    let t3248 = t1014 * t1127;
    let t3250 = t126 * t88;
    let t3251 = t85 * t3250;
    (t3246, t3247, t3248, t3250, t3251)
}
