//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1386/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1386(t12007: f64, t12089: f64, t12103: f64, t12110: f64, t1402: f64, t1429: f64, t1508: f64, t1599: f64, t1641: f64, t193: f64, t30708: f64, t30712: f64, t34486: f64, t34489: f64, t34492: f64, t34498: f64, t34500: f64, t34503: f64, t34510: f64, t34512: f64, t3701: f64, t3710: f64, t4428: f64, t4634: f64) -> f64 {
    let t38588 = -t30708 - t34486 - t34489 - t34492 + 0.35750489951850426669e0_f64 * t1508 * t3701 * t193 - 0.23005755572352449806e1_f64 * t4634 * t3710 - 0.46011511144704899612e1_f64 * t1641 * t12110 - 0.71500979903700853338e0_f64 * t1599 * t12103 - t34498 - t34500 - t34503 + t34510 + t34512 + 0.1022478025437886658e1_f64 * t4428 * t12089 - t30712 - 0.92686455430723328401e-1_f64 * t1429 * t1402 * t12007;
    t38588
}
