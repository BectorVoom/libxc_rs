//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 617/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk617(t719: f64, t4972: f64, t746: f64, t741: f64, t4803: f64, t641: f64, t5275: f64, t5279: f64, t5281: f64, t5287: f64, t5292: f64, t5296: f64, t5300: f64, t5304: f64, t5308: f64, t5311: f64, t5313: f64, t5318: f64, t5324: f64, t5328: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5330 = 1.0_f64 / t719;
    let t5331 = t5330 * t4972;
    let t5332 = t746 * t5331;
    let t5333 = t741 * t5332;
    let t5335 = t641 * t4803;
    let t5336 = t746 * t5335;
    let t5337 = t741 * t5336;
    let t5339 = t5275 / 16.0_f64 - t5279 / 8.0_f64 + t5281 / 12.0_f64 + t5287 / 8.0_f64 - t5292 / 12.0_f64 - t5296 / 16.0_f64 - t5300 / 72.0_f64 + t5304 / 24.0_f64 - t5308 / 256.0_f64 + t5311 / 128.0_f64 - t5313 / 96.0_f64 - t5318 / 128.0_f64 + t5324 / 96.0_f64 + t5328 / 256.0_f64 - t5333 / 576.0_f64 - t5337 / 192.0_f64;
    (t5330, t5332, t5333, t5336, t5337, t5339)
}
