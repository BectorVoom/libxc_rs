//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1260/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1260(t1791: f64, t19345: f64, t5790: f64, t6090: f64, t1675: f64, t5791: f64, t6073: f64, t19380: f64, t1792: f64, t18350: f64, t18648: f64, t18652: f64, t18661: f64, t18666: f64, t18671: f64, t18673: f64, t18676: f64, t19342: f64, t19349: f64, t19352: f64, t5483: f64, t5794: f64, t6304: f64) -> (f64, f64, f64, f64) {
    let t20264 = t1791 * t19345;
    let t20275 = t5790 * t6090;
    let t20276 = t1675 * t20275;
    let t20278 = t6073 * t5791;
    let t20282 = t1791 * t19380;
    let t20285 = 40.0_f64 / 9.0_f64 * t18671 + 16.0_f64 / 9.0_f64 * t18676 + 10.0_f64 * t18666 * t19342 + 10.0_f64 / 3.0_f64 * t18350 * t20264 + t18648 - 8.0_f64 / 9.0_f64 * t18652 - 8.0_f64 / 9.0_f64 * t18661 + 10.0_f64 / 3.0_f64 * t19349 * t18673 + t19352 * t1792 / 3.0_f64 + t6073 * t5794 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t20276 - 8.0_f64 / 9.0_f64 * t20278 + t5483 * t6304 / 3.0_f64 + t1675 * t20282 / 3.0_f64;
    (t20264, t20275, t20282, t20285)
}
