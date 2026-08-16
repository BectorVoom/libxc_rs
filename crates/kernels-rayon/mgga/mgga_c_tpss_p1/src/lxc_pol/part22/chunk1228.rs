//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1228/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1228(t5895: f64, t645: f64, t18434: f64, t18461: f64, t18437: f64, t18440: f64, t18442: f64, t18447: f64, t18451: f64, t18455: f64, t18457: f64, t18459: f64, t18465: f64, t18467: f64, t18469: f64) -> (f64, f64, f64, f64) {
    let t18930 = t5895 * t645;
    let t18934 = 35.0_f64 / 216.0_f64 * t18434;
    let t18943 = 119.0_f64 / 3456.0_f64 * t18461;
    let t18947 = t18934 + 7.0_f64 / 36.0_f64 * t18437 + t18440 / 8.0_f64 - t18442 / 24.0_f64 + t18447 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t18451 + t18455 / 96.0_f64 - t18457 / 768.0_f64 - t18459 / 768.0_f64 + t18943 + 7.0_f64 / 144.0_f64 * t18465 + 5.0_f64 / 192.0_f64 * t18467 - t18469 / 192.0_f64;
    (t18930, t18934, t18943, t18947)
}
