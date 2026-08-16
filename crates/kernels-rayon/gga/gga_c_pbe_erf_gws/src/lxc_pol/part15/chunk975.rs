//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 975/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk975(t1114: f64, t6159: f64, t6154: f64, t2362: f64, t2397: f64, t3083: f64, t2373: f64, t3066: f64, t4390: f64, t4425: f64, t4430: f64, t4443: f64, t4454: f64, t4467: f64, t4469: f64, t4484: f64, t6164: f64, t833: f64, t8629: f64, t8634: f64, t8641: f64, t8643: f64, t8646: f64, t8649: f64, t8654: f64) -> f64 {
    let t8659 = t1114 * t6159;
    let t8662 = t1114 * t6154;
    let t8664 = 7.0_f64 / 144.0_f64 * t8662 * t2362;
    let t8666 = 7.0_f64 / 144.0_f64 * t3083 * t2397;
    let t8667 = t8629 * t4390 / 24.0_f64 + t8629 * t4484 / 48.0_f64 + t8634 * t833 / 48.0_f64 + 35.0_f64 / 216.0_f64 * t4425 - 35.0_f64 / 216.0_f64 * t4430 - 35.0_f64 / 108.0_f64 * t4443 + t8641 + t8643 - 7.0_f64 / 288.0_f64 * t4454 + t8646 + t3066 * t8649 / 24.0_f64 - t8654 * t2373 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t4467 + 7.0_f64 / 72.0_f64 * t4469 + t8659 * t6164 / 48.0_f64 + t8664 - t8666;
    t8667
}
