//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 952/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk952(t114034: f64, t114046: f64, t31560: f64, t6914: f64, t31590: f64, t6883: f64, t22724: f64, t31594: f64, t2085: f64, t213: f64, t225: f64, t22642: f64, t22643: f64, t8621: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115464 = 7.0_f64 / 576.0_f64 * t114034;
    let t115467 = 0.5383034145885385447e-3_f64 * t114046;
    let t115508 = t6914 * t31560;
    let t115530 = t6883 * t31590;
    let t115539 = t22724 * t31594;
    let t115545 = t213 * t2085 * t225;
    let t115550 = t22642 * t22643 * t8621;
    (t115464, t115467, t115508, t115530, t115539, t115545, t115550)
}
