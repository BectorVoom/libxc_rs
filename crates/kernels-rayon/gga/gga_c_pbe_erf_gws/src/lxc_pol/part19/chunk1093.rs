//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1093/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1093(t3886: f64, t810: f64, t2376: f64, t2409: f64, t3893: f64, t840: f64, t3721: f64, t3067: f64, t1115: f64, t12199: f64, t12201: f64, t12206: f64, t12210: f64, t12215: f64, t12220: f64, t12223: f64, t12229: f64, t12234: f64, t12239: f64, t12243: f64, t12246: f64, t2408: f64, t3066: f64, t3079: f64, t3207: f64, t827: f64, t833: f64, t9695: f64, t9701: f64, t9709: f64) -> (f64, f64, f64, f64, f64) {
    let t12248 = t3886 * t810;
    let t12250 = t2409 * t2376 * t12248;
    let t12253 = t840 * t3893;
    let t12255 = t3721 * t810;
    let t12257 = t2409 * t3067 * t12255;
    let t12260 = -7.0_f64 / 288.0_f64 * t12199 + t12201 * t833 / 96.0_f64 - t3066 * t12206 / 16.0_f64 + t3207 * t12210 / 8.0_f64 + t3066 * t12215 / 24.0_f64 + t12220 * t3079 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t12223 - t1115 * t9709 / 48.0_f64 + t12229 * t3079 / 96.0_f64 + t827 * t12234 / 96.0_f64 + t9695 - t9701 - t3207 * t12239 / 16.0_f64 + t3207 * t12243 / 16.0_f64 + 7.0_f64 / 288.0_f64 * t12246 + t2408 * t12250 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t12253 - t2408 * t12257 / 24.0_f64;
    (t12248, t12250, t12255, t12257, t12260)
}
