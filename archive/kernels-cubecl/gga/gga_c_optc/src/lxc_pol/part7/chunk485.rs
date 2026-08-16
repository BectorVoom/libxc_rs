//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 485/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk485<F: Float>(t2409: F, t799: F, t779: F, t1002: F, t1015: F, t2248: F, t2254: F, t2320: F, t2333: F, t2337: F, t2344: F, t2348: F, t2354: F, t2357: F, t2360: F, t2364: F, t2369: F, t2377: F, t2380: F, t355: F, t364: F, t984: F, t989: F) -> (F, F, F) {
    let t2410 = t2409 * t799;
    let t2412 = F::cast_from(1.0_f64) * t779 * t2410;
    let t2413 = -F::cast_from(50.0_f64) / F::cast_from(3.0_f64) * t2248 * t1015 + F::cast_from(100.0_f64) / F::cast_from(81.0_f64) * t2254 + t2320 * t364 / F::cast_from(2.0_f64) + F::cast_from(20000.0_f64) / F::cast_from(81.0_f64) * t2333 * t2337 + F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t355 * t2344 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2348 - t2354 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t984 * t989 + t2357 / F::cast_from(3.0_f64) + t2360 * t1002 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2364 * t1002 + t2369 / F::cast_from(9.0_f64) - t2377 + t2380 + t2412;
    (t2410, t2412, t2413)
}
