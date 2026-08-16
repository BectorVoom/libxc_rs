//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1326/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1326(t27785: f64, t2822: f64, t14570: f64, t2175: f64, t26685: f64, t3489: f64, t7693: f64, t93171: f64, t93173: f64, t93704: f64, t93714: f64, t95860: f64, t96372: f64, t96379: f64, t96382: f64, t96388: f64, t96391: f64) -> (f64, f64) {
    let t96395 = t2822 * t27785;
    let t96396 = 0.14739506172839506172e-2_f64 * t96395;
    let t96397 = -0.92754700520833333333e-4_f64 * t26685 * t96372 + 0.18550940104166666667e-3_f64 * t26685 * t95860 - 0.3684876543209876543e-3_f64 * t93171 + 0.66327777777777777776e-2_f64 * t96379 - 0.15445601851851851852e-3_f64 * t96382 + 0.37069444444444444444e-2_f64 * t14570 * t3489 * t2175 - 0.46336805555555555556e-3_f64 * t93704 - 0.20612155671296296296e-4_f64 * t96388 - 0.16489724537037037037e-3_f64 * t93714 - 0.4946917361111111111e-3_f64 * t96391 * t7693 - 0.22109259259259259258e-2_f64 * t93173 + t96396;
    (t96395, t96397)
}
