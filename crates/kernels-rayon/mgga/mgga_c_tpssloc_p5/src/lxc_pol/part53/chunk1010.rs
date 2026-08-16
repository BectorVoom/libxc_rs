//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1010/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1010(t193: f64, t8756: f64, t200: f64, t8743: f64, t1877: f64, t2219: f64, t8744: f64, t776: f64, t7844: f64, t1530: f64, t7109: f64, t116481: f64, t118377: f64, t118407: f64, t1408: f64, t22960: f64, t24191: f64, t24339: f64, t25: f64, t25015: f64, t25024: f64, t25028: f64, t2522: f64, t25373: f64, t25377: f64, t25381: f64, t25392: f64, t26739: f64, t26756: f64, t32034: f64, t32047: f64, t33991: f64, t34004: f64, t606: f64, t7114: f64, t8748: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123378 = t193 * t8756;
    let t123382 = t193 * t200 * t8743;
    let t123398 = t1877 * t8744 * t2219;
    let t123414 = t7844 * t776;
    let t123418 = t1530 * t7109;
    let t123428 = 3.0_f64 * t116481 * t118407 - 3.0_f64 * t123378 * t118377 + 3.0_f64 * t123382 * t25015 + t1877 * t33991 * t606 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t8744 * t25024 - t1877 * t32034 * t25377 / 2.0_f64 - t1877 * t32034 * t25392 / 2.0_f64 + t123398 + 3.0_f64 / 2.0_f64 * t2522 * t8744 * t25028 + t1877 * t32047 * t25392 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t25024 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t25028 - t1877 * t24339 * t34004 + t1877 * t32047 * t25381 - 3.0_f64 * t24191 * t22960 * t123414 + 2.0_f64 * t26756 * t25373 * t123418 - t1877 * t7114 * t1408 * t7109 - t1877 * t7114 * t25 * t26739;
    (t123378, t123382, t123398, t123414, t123418, t123428)
}
