//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1483/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1483(t1735: f64, t3252: f64, t3578: f64, t3248: f64, t11642: f64, t11644: f64, t11649: f64, t1174: f64, t1227: f64, t15434: f64, t15438: f64, t15446: f64, t15448: f64, t15450: f64, t15452: f64, t15455: f64, t3518: f64, t3527: f64, t3531: f64, t3577: f64, t5005: f64) -> f64 {
    let t15458 = t1735 * t3252;
    let t15459 = t3578 * t15458;
    let t15462 = t1735 * t3248;
    let t15463 = t3578 * t15462;
    let t15466 = t11642 / 4608.0_f64 - t11644 / 6912.0_f64 + t11649 - 7.0_f64 / 648.0_f64 * t1174 * t15434 - t15438 * t3518 / 3072.0_f64 - t5005 * t3527 / 4608.0_f64 - t5005 * t3531 / 2304.0_f64 + t15446 - t15448 - t15450 + t15452 - 5.0_f64 / 5184.0_f64 * t1227 * t15455 - t3577 * t15459 / 4608.0_f64 - t3577 * t15463 / 2304.0_f64;
    t15466
}
