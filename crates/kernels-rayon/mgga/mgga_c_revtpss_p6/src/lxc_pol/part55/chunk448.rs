//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 448/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk448(t2455: f64, t2457: f64, t2454: f64, t786: f64, t861: f64, t789: f64, t252: f64, t867: f64, t676: f64, t886: f64, t123: f64, t215: f64, t685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2458 = t2455 * t2457;
    let t2460 = 0.11565819519348392139e-2_f64 * t2454 * t2458;
    let t2461 = t786 * t861;
    let t2462 = t2461 * t789;
    let t2464 = t252 * t867;
    let t2465 = t786 * t2464;
    let t2466 = t676 * t886;
    let t2467 = t123 * t2466;
    let t2468 = t2465 * t2467;
    let t2470 = t685 * t215;
    (t2458, t2460, t2462, t2465, t2466, t2467, t2468, t2470)
}
