//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 578/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk578(t2482: f64, t866: f64, t2481: f64, t267: f64, t270: f64, t849: f64, t2453: f64, t2455: f64, t2462: f64, t2467: f64, t2471: f64, t847: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2483 = t2482 * t866;
    let t2485 = 2.0_f64 * t2481 * t2483;
    let t2487 = 1.0_f64 / t270 / t267;
    let t2488 = t849 * t849;
    let t2489 = t2487 * t2488;
    let t2491 = 4.0_f64 / 9.0_f64 * t2453;
    let t2496 = t2491 + 2.0_f64 / 9.0_f64 * t2455 - 2.0_f64 / 9.0_f64 * t2462 + 2.0_f64 / 3.0_f64 * t2467 - t2471 / 3.0_f64;
    let t2497 = t847 * t2496;
    (t2483, t2485, t2487, t2488, t2489, t2491, t2496, t2497)
}
