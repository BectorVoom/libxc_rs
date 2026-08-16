//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1389/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1389(t107: f64, t12012: f64, t1339: f64, t1415: f64, t1417: f64, t1520: f64, t30705: f64, t34445: f64, t34449: f64, t34454: f64, t34458: f64, t34462: f64, t34465: f64, t34467: f64, t34470: f64, t34473: f64, t34477: f64, t34484: f64, t3702: f64, t3705: f64, t38271: f64, t4631: f64, t4811: f64, t590: f64) -> f64 {
    let t38573 = -t34445 - t34449 - t34454 + t34458 + 0.79445533226334281486e-1_f64 * t1415 * t12012 * t107 * t1417 + 0.1022478025437886658e1_f64 * t4811 * t1339 * t38271 * t590 - t34462 + t34465 - 0.35750489951850426669e0_f64 * t4631 * t3705 - 0.79445533226334281487e-1_f64 * t3702 * t1520 + t34467 + t34470 - t34473 + t34477 + t34484 + t30705;
    t38573
}
