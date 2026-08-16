//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1381/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1381(t34491: f64, t10604: f64, t1415: f64, t1646: f64, t30705: f64, t30708: f64, t34454: f64, t34458: f64, t34462: f64, t34465: f64, t34467: f64, t34470: f64, t34473: f64, t34477: f64, t34478: f64, t34484: f64, t34486: f64, t34489: f64, t4425: f64) -> f64 {
    let t34492 = 0.38342925953920749676e0_f64 * t34491;
    let t34493 = -t34454 - 0.51123901271894332905e0_f64 * t4425 * t10604 + t34458 - t34462 + t34465 + t34467 + t34470 - t34473 + t34477 - 0.71500979903700853338e0_f64 * t1415 * t34478 * t1646 + t34484 + t30705 - t30708 - t34486 - t34489 - t34492;
    t34493
}
