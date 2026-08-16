//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2714/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2714(t109: f64, t45421: f64, t45422: f64, t45656: f64, t45659: f64, t45689: f64, t55531: f64, t55537: f64, t55546: f64, t55559: f64, t55561: f64, t75699: f64, t1268: f64, t12725: f64, t1458: f64, t19451: f64, t19456: f64, t19534: f64, t20347: f64, t2314: f64, t26114: f64, t26117: f64, t28002: f64, t4028: f64, t4072: f64, t5113: f64, t5493: f64, t55943: f64, t67001: f64, t671: f64, t75275: f64, t75555: f64, t75560: f64, t7676: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t75701 = piecewise3(t110, 0.0_f64, t45421 + 154.0_f64 / 27.0_f64 * t45422 + 154.0_f64 / 9.0_f64 * t45656 + t45659 - t45689 + 22.0_f64 / 3.0_f64 * t55537 + 6.0_f64 * t55546 - 4.0_f64 * t55561 - 11.0_f64 / 3.0_f64 * t55531 - 2.0_f64 * t55559 + t75699);
    let t75704 = 2.0_f64 * t1268 * t75701 + 6.0_f64 * t12725 * t5493 + 6.0_f64 * t1458 * t55943 + 6.0_f64 * t1458 * t75560 + 6.0_f64 * t19451 * t4072 + 6.0_f64 * t19456 * t5493 + 6.0_f64 * t19534 * t4028 + 6.0_f64 * t19534 * t7676 + 2.0_f64 * t20347 * t2314 + 2.0_f64 * t20347 * t5113 + 6.0_f64 * t26114 * t5493 + 6.0_f64 * t26117 * t5493 + 12.0_f64 * t28002 * t4072 + 2.0_f64 * t67001 * t671 + 6.0_f64 * t75275 + t75555;
    (t75701, t75704)
}
