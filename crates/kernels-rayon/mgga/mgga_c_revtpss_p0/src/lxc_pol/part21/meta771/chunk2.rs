//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2740/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2740(t10696: f64, t1544: f64, t14832: f64, t2394: f64, t2661: f64, t40409: f64, t40411: f64, t40413: f64, t40421: f64, t40425: f64, t40429: f64, t50151: f64, t50370: f64, t50372: f64, t50375: f64, t50377: f64, t50381: f64, t50383: f64, t50385: f64, t50387: f64, t50390: f64, t50394: f64, t828: f64, t851: f64, t855: f64) -> (f64, f64) {
    let t50396 = t10696 * t1544;
    let t50399 = t2661 * t14832 * t50396 * t2394;
    let t50408 = -0.60246173160355784832e-6_f64 * t40409 + 0.18292914397043087775e-2_f64 * t40411 + 0.10003937560882938627e-2_f64 * t40413 + 0.15117061203111996147e0_f64 * t50370 + 0.72250660161932334527e-3_f64 * t50372 - t50375 - 0.80328230880474379779e-6_f64 * t50377 + 0.11294745624363664198e-6_f64 * t50381 - 0.68026775414003982662e-1_f64 * t50383 - 0.51384669507166276316e-2_f64 * t50385 + 0.45732285992607719437e-2_f64 * t50387 + t50390 - 0.85748036236139473944e-3_f64 * t50394 + 0.25724410870841842183e-2_f64 * t50399 - 0.85748036236139473944e-3_f64 * t851 * t855 * t828 * t50151 + 0.76230004213927992336e-3_f64 * t40421 - 0.38538502130374707238e-2_f64 * t40425 + 0.21437009059034868486e-4_f64 * t40429;
    (t50396, t50408)
}
