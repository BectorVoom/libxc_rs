//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2714/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2714<F: Float>(t109: F, t45421: F, t45422: F, t45656: F, t45659: F, t45689: F, t55531: F, t55537: F, t55546: F, t55559: F, t55561: F, t75699: F, t1268: F, t12725: F, t1458: F, t19451: F, t19456: F, t19534: F, t20347: F, t2314: F, t26114: F, t26117: F, t28002: F, t4028: F, t4072: F, t5113: F, t5493: F, t55943: F, t67001: F, t671: F, t75275: F, t75555: F, t75560: F, t7676: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t75701 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t45421 + F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t45422 + F::cast_from(154.0_f64) / F::cast_from(9.0_f64) * t45656 + t45659 - t45689 + F::cast_from(22.0_f64) / F::cast_from(3.0_f64) * t55537 + F::cast_from(6.0_f64) * t55546 - F::cast_from(4.0_f64) * t55561 - F::cast_from(11.0_f64) / F::cast_from(3.0_f64) * t55531 - F::cast_from(2.0_f64) * t55559 + t75699);
    let t75704 = F::cast_from(2.0_f64) * t1268 * t75701 + F::cast_from(6.0_f64) * t12725 * t5493 + F::cast_from(6.0_f64) * t1458 * t55943 + F::cast_from(6.0_f64) * t1458 * t75560 + F::cast_from(6.0_f64) * t19451 * t4072 + F::cast_from(6.0_f64) * t19456 * t5493 + F::cast_from(6.0_f64) * t19534 * t4028 + F::cast_from(6.0_f64) * t19534 * t7676 + F::cast_from(2.0_f64) * t20347 * t2314 + F::cast_from(2.0_f64) * t20347 * t5113 + F::cast_from(6.0_f64) * t26114 * t5493 + F::cast_from(6.0_f64) * t26117 * t5493 + F::cast_from(12.0_f64) * t28002 * t4072 + F::cast_from(2.0_f64) * t67001 * t671 + F::cast_from(6.0_f64) * t75275 + t75555;
    (t75701, t75704)
}
