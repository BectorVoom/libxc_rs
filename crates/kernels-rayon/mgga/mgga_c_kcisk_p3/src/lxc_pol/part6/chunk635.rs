//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 635/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk635(t645: f64, t1755: f64, t8786: f64, t2436: f64, t2442: f64, t340: f64, t639: f64, t642: f64, t8773: f64, t8781: f64, t655: f64, t2364: f64, t2464: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t646 = t645 < -0.66725e-1_f64;
    let t8787 = t1755 * t8786;
    let t8792 = piecewise3(t646, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t8773 * t642 - 20.0_f64 / 27.0_f64 * t340 * t2436 * t2442 + 40.0_f64 / 81.0_f64 * t340 * t639 * t8781 - 10.0_f64 / 27.0_f64 * t340 * t639 * t8787);
    let t8793 = t8792 * sigma2;
    let t8794 = t8793 * t655;
    let t8797 = t2364 * t2464;
    (t8787, t8793, t8794, t8797)
}
