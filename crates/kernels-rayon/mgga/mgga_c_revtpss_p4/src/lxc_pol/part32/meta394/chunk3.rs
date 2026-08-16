//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1364/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1364(t18285: f64, t18297: f64, t150: f64, t190: f64, t5944: f64, t750: f64, t189: f64, t5825: f64, t606: f64, t4401: f64, t10552: f64, t10554: f64, t14317: f64, t18253: f64, t18256: f64, t18261: f64, t18262: f64, t18265: f64, t18267: f64, t18268: f64, t1940: f64, t2403: f64, t4537: f64, t4541: f64, t4556: f64, t775: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64) {
    let t18298 = t18285 + t18297;
    let t18299 = t150 * t18298;
    let t18300 = t18299 * t190;
    let t18301 = t5944 * t750;
    let t18305 = t189 * t5825;
    let t18306 = t18305 * t606;
    let t18308 = 12.0_f64 * t4401 * t18306;
    let t18309 = -3.0_f64 * t18268 * t2403 * t775 - 2.0_f64 * t1940 * t4537 * t4556 + 12.0_f64 * t18253 * t4541 + 6.0_f64 * t18256 * t4541 - t10552 + t10554 + t14317 + t18261 + t18262 + t18265 + t18267 + t18300 + t18301 + t18308 - t9278 + t9308 + t9316 + t9329 + t9333;
    (t18298, t18300, t18301, t18308, t18309)
}
