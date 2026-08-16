//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 326/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk326(t4370: f64, t898: f64, t900: f64, t2265: f64, t2912: f64, t2913: f64, t2915: f64, t3628: f64, t4332: f64, t4335: f64, t4339: f64, t4343: f64, t4347: f64, t4350: f64, t4354: f64, t4359: f64, t631: f64) -> f64 {
    let t4372 = t898 * t900 * t4370;
    let t4375 = -t2912 - t2913 / 9.0_f64 - t2915 / 3.0_f64 - t4332 / 9.0_f64 + t2265 * t4335 / 18.0_f64 - t2265 * t4339 / 3.0_f64 - t2265 * t4343 / 3.0_f64 - t3628 * t4347 / 3.0_f64 - t4350 / 3.0_f64 - t2265 * t4354 / 3.0_f64 - 3.0_f64 / 2.0_f64 * t631 * t4359 + t631 * t4372 / 2.0_f64;
    t4375
}
