//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 527/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk527(t2815: f64, t3338: f64, t3337: f64, t1130: f64, t3190: f64, t376: f64, t375: f64, t359: f64, t3219: f64, t387: f64, t382: f64, t280: f64, t383: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3339 = t3338 * t2815;
    let t3340 = t3337 * t3339;
    let t3342 = t1130 * t3190;
    let t3343 = t376 * t3342;
    let t3344 = t375 * t3343;
    let t3346 = 1.0_f64 / t359;
    let t3347 = t3346 * t3219;
    let t3348 = t387 * t3347;
    let t3349 = t382 * t3348;
    let t3353 = 1.0_f64 / t280 / t383 / t980;
    (t3339, t3340, t3343, t3344, t3346, t3348, t3349, t3353)
}
