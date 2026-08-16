//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 406/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk406(t2526: f64, t62: f64, t755: f64, t752: f64, t2379: f64, t2381: f64, t2386: f64, t2390: f64, t2416: f64, t2423: f64, t2427: f64, t2430: f64, t2482: f64, t2486: f64, t2494: f64, t688: f64, t707: f64, t82: f64) -> (f64, f64, f64, f64) {
    let t2527 = t62 * t2526;
    let t2528 = t755 * t2527;
    let t2529 = t752 * t2528;
    let t2531 = t2379 * t82 - 0.13345e0_f64 * t2381 * t707 + 0.890445125e-2_f64 * t2386 * t2390 - 0.66725e-1_f64 * t688 * t2416 + 0.66725e-1_f64 * t688 * t2390 + 0.30952962962962962962e-1_f64 * t2423 - 0.2653111111111111111e-1_f64 * t2427 + 0.2653111111111111111e-1_f64 * t2430 + 0.99491666666666666664e-2_f64 * t2482 - 0.19898333333333333333e-1_f64 * t2486 + 0.19898333333333333333e-1_f64 * t2494 - 0.99491666666666666664e-2_f64 * t2529;
    (t2527, t2528, t2529, t2531)
}
