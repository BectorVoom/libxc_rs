//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 119/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk119(t349: f64, t381: f64, t362: f64, t68: f64, t353: f64, t254: f64, t193: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t382 = t349 * t381;
    let t383 = t68 * t362;
    let t384 = t383 * t381;
    let t386 = t353 * t384 + 1.0_f64;
    let t387 = 1.0_f64 / t386;
    let t388 = t254 * t387;
    let t390 = t382 * t388 + 1.0_f64;
    let t391 = f64::ln(t390);
    let t394 = t193 * t336 * t391 - t293 + t328 + t330;
    let t395 = t265 < t394;
    (t382, t383, t384, t386, t388, t390, t394)
}
