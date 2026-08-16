//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 780/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk780(t4365: f64, t4366: f64, t4364: f64, t1544: f64, t854: f64, t236: f64, t807: f64, t2498: f64, t2518: f64, t2522: f64, t2526: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t4300: f64, t4301: f64, t4304: f64) -> (f64, f64, f64, f64, f64) {
    let t4367 = t4365 * t4366;
    let t4368 = t4364 * t4367;
    let t4371 = t854 * t1544;
    let t4372 = t236 * t4371;
    let t4373 = t807 * t4372;
    let t4376 = t4300 - t2569 + t2579 + t2587 - t2522 - t2498 - t2518 - t4301 + t2526 + t2610 - t4304 - t2562;
    (t4368, t4371, t4372, t4373, t4376)
}
