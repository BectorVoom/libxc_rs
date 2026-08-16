//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1390/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1390(t34642: f64, t10514: f64, t21370: f64, t10531: f64, t10534: f64, t1406: f64, t10557: f64, t6795: f64, t8072: f64, t9285: f64, t204: f64, t2476: f64, t34567: f64, t34621: f64, t34623: f64, t34626: f64, t34628: f64, t34631: f64, t34634: f64, t34636: f64, t34638: f64, t34640: f64) -> f64 {
    let t34643 = 0.89376224879626066674e-1_f64 * t34642;
    let t34645 = 0.12423108009070322895e3_f64 * t21370 * t10514;
    let t34648 = 0.55213813373645879534e2_f64 * t1406 * t10531 * t10534;
    let t34650 = 0.42900587942220512003e1_f64 * t10557 * t6795;
    let t34652 = 0.71500979903700853338e0_f64 * t9285 * t8072;
    let t34656 = t34621 + t34623 + t34626 + t34628 + t34631 + t34634 + t34636 + t34638 - t34640 - t34643 - t34645 + t34648 + t34650 + t34652 + 0.92023022289409799224e1_f64 * t2476 * t204 * t34567;
    t34656
}
