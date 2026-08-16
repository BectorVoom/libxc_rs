//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1159/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1159(t1153: f64, t15478: f64, t15481: f64, t15484: f64, t15601: f64, t15605: f64, t15607: f64, t15609: f64, t15612: f64, t15615: f64, t15618: f64, t15621: f64, t15625: f64, t15628: f64, t15632: f64, t15634: f64, t15637: f64, t15639: f64, t15794: f64, t16015: f64, t198: f64, t330: f64, t4023: f64, t4325: f64, t4329: f64) -> f64 {
    let t16022 = t1153 * t16015 * t198 * t330 - 2.0_f64 * t4023 * t4325 * t4329 - t15478 - t15481 - t15484 - t15601 + t15605 - t15607 + t15609 + t15612 - t15615 - t15618 - t15621 + t15625 + t15628 + t15632 + t15634 + t15637 + t15639 + t15794;
    t16022
}
