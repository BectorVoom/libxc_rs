//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 916/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk916<F: Float>(t4049: F, t5396: F, t20215: F, t95: F, t5415: F, t1449: F, t5480: F, t9398: F, t4059: F, t5484: F, t103: F, t100: F, t104: F, t1447: F, t1450: F, t20312: F, t5475: F, t5481: F, t5485: F, t92: F, tau1: F) -> F {
    let t20315 = t4049 * t5396;
    let t20318 = F::cast_from(3.0_f64) * t20215;
    let t20319 = t95 * t20318;
    let t20322 = tau1 * t5415;
    let t20331 = t5480 * t1449;
    let t20332 = t9398 * t20331;
    let t20335 = t4059 * t5484;
    let t20338 = -t20318;
    let t20339 = t103 * t20338;
    let t20342 = -F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t92 * t20312 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t92 * t20315 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t20319 - F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t20322 * t104 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t5475 * t1450 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1447 * t5481 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t1447 * t5485 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t100 * t20332 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t100 * t20335 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t20339;
    t20342
}
