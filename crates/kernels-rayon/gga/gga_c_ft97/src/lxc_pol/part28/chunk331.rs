//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 331/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk331(t1023: f64, t1526: f64, t1527: f64, t1942: f64, t342: f64, t343: f64, t4641: f64, t4645: f64, t1537: f64, t2: f64, t4: f64, t26: f64) -> (f64, f64, f64, f64) {
    let t4649 = t1023 - t1942 - t1526 * t1527 * t4641 / 12.0_f64 - t342 * t343 * t4645 / 4.0_f64;
    let t5493 = t1537 * t2;
    let t5494 = t5493 * t4;
    let t5495 = t5494 * t26;
    (t4649, t5493, t5494, t5495)
}
