//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 583/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk583(t284: f64, t2482: f64, t2531: f64, t2453: f64, t2455: f64, t2462: f64, t2467: f64, t2471: f64, t872: f64, t876: f64, t301: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2532 = t284 * t284;
    let t2533 = 1.0_f64 / t2532;
    let t2534 = t2482 * t2533;
    let t2536 = 0.16081979498692535067e2_f64 * t2531 * t2534;
    let t2537 = 0.22831111111111111111e-1_f64 * t2453;
    let t2542 = t2537 + 0.11415555555555555555e-1_f64 * t2455 - 0.11415555555555555555e-1_f64 * t2462 + 0.34246666666666666666e-1_f64 * t2467 - 0.17123333333333333333e-1_f64 * t2471;
    let t2545 = t872 * t876;
    let t2548 = t875 * t301;
    (t2532, t2533, t2534, t2536, t2537, t2542, t2545, t2548)
}
