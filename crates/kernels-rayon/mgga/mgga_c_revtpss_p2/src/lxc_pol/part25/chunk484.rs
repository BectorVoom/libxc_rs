//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 484/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk484(t676: f64, t762: f64, t2629: f64, t2392: f64, t2400: f64, t2402: f64, t2416: f64, t2498: f64, t2518: f64, t2522: f64, t2525: f64, t2527: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2614: f64, t2617: f64, t2621: f64, t2624: f64, t2628: f64) -> (f64, f64, f64) {
    let t2630 = t676 * t762;
    let t2632 = 0.10843581300301739842e-1_f64 * t2629 * t2630;
    let t2633 = -t2498 - t2518 - t2522 - t2525 + t2402 + t2527 + t2610 + t2579 + t2587 + t2614 + t2416 - t2562 + t2400 + t2617 - t2569 + t2621 - t2624 + t2628 + t2632 + t2392;
    (t2630, t2632, t2633)
}
