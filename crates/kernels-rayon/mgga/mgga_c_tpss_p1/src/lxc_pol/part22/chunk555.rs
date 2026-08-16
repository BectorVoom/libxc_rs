//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 555/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk555(t2383: f64, t803: f64, t206: f64, t237: f64, t235: f64, t72: f64, t2116: f64, t774: f64, t2133: f64, t801: f64, t2142: f64, t2144: f64, t2147: f64, t2149: f64, t2153: f64, t2160: f64, t2165: f64, t2170: f64, t2173: f64, t2179: f64, t2367: f64, t2372: f64, t2381: f64, t761: f64, t771: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2384 = t2383 * t803;
    let t2387 = 1.0_f64 / t237 / t206;
    let t2388 = t235 * t2387;
    let t2389 = t2388 * t72;
    let t2391 = t2389 * t774 * t2116;
    let t2395 = t801 * t774 * t2133;
    let t2398 = t2142 + 7.0_f64 / 72.0_f64 * t2144 + t2147 * t2149 / 16.0_f64 - t761 * t2153 / 48.0_f64 + t2160 * t2165 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t2170 + t2173 * t2179 / 384.0_f64 - t771 * t2367 / 3072.0_f64 - t771 * t2372 / 3072.0_f64 + t2381 + 7.0_f64 / 576.0_f64 * t2384 + 5.0_f64 / 768.0_f64 * t797 * t2391 - t797 * t2395 / 768.0_f64;
    (t2384, t2387, t2389, t2391, t2395, t2398)
}
