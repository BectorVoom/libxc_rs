//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1401/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1401(t33483: f64, t776: f64, t1877: f64, t2219: f64, t8566: f64, t101840: f64, t118410: f64, t24191: f64, t24339: f64, t2522: f64, t25373: f64, t25381: f64, t25392: f64, t31434: f64, t31441: f64, t31449: f64, t32899: f64, t33466: f64, t33477: f64, t33484: f64, t6542: f64, t7114: f64, t84797: f64, t8569: f64, t86721: f64, t92271: f64, t92276: f64) -> (f64, f64, f64) {
    let t121837 = t33483 * t776;
    let t121861 = t1877 * t8566 * t2219;
    let t121865 = -t1877 * t92276 * t8569 / 2.0_f64 + 3.0_f64 * t24191 * t25373 * t121837 + t92271 * t33484 - 3.0_f64 / 2.0_f64 * t24191 * t86721 * t31441 - t1877 * t24339 * t32899 / 2.0_f64 - t1877 * t7114 * t118410 / 2.0_f64 - t1877 * t31434 * t25381 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t33466 * t6542 - t1877 * t31434 * t25392 / 2.0_f64 + t121861 - 3.0_f64 / 2.0_f64 * t84797 * t33477 + t101840 * t31449;
    (t121837, t121861, t121865)
}
