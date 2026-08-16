//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 480/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk480(t2327: f64, t598: f64, t545: f64, t615: f64, t2179: f64, t2180: f64, t2182: f64, t2183: f64, t2184: f64, t2185: f64, t2189: f64, t2190: f64, t2191: f64, t2258: f64, t2261: f64, t2265: f64, t2269: f64, t2271: f64, t2275: f64, t2279: f64, t2283: f64, t2285: f64) -> (f64, f64, f64) {
    let t2328 = t598 * t2327;
    let t2338 = t615 * t545;
    let t2372 = t2179 - t2180 + t2182 - t2183 - t2184 - t2185 - 0.34299214494455789578e-2_f64 * t2258 - t2189 + t2190 + t2261 / 48.0_f64 - 0.21437009059034868486e-3_f64 * t2265 + 0.31448092289604152069e-3_f64 * t2269 + t2191 - t2271 / 48.0_f64 - t2275 / 64.0_f64 - t2279 / 192.0_f64 - 0.7640625e-2_f64 * t2283 - t2285 / 24.0_f64;
    (t2328, t2338, t2372)
}
