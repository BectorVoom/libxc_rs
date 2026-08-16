//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1155/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1155<F: Float>(t6467: F, t743: F, t1056: F, t18677: F, t18672: F, t345: F, t18653: F, t1154: F, t18648: F, t18657: F, t1079: F, t104: F, t111: F, t120: F, t19423: F, t19425: F, t19427: F, t19430: F, t19433: F, t19436: F, t19438: F, t19440: F, t4858: F, t4881: F) -> F {
    let t19442 = t743 * t6467;
    let t19444 = t1056 * t18677;
    let t19447 = t345 * t18672;
    let t19450 = t345 * t18653;
    let t19453 = t1154 * t18648;
    let t19456 = t345 * t18657;
    let t19459 = t1079 * t18677;
    let t19462 = t1056 * t18672;
    let t19465 = t345 * t18648;
    let t19468 = t1056 * t18657;
    let t19471 = F::cast_from(0.4684e-2_f64) * t19423 - F::cast_from(0.15613333333333333333e-2_f64) * t19425 - F::cast_from(0.3513e-2_f64) * t104 * t19427 + F::cast_from(0.7925e-3_f64) * t111 * t19430 + F::cast_from(0.50413125e-5_f64) * t120 * t19433 + F::cast_from(0.15684083333333333333e-4_f64) * t19436 - F::cast_from(0.13208333333333333333e-2_f64) * t19438 + F::cast_from(0.88055555555555555555e-3_f64) * t19440 - F::cast_from(0.117630625e-4_f64) * t19442 + F::cast_from(0.7026e-2_f64) * t104 * t19444 + F::cast_from(0.1171e-2_f64) * t104 * t19447 - F::cast_from(0.7026e-2_f64) * t104 * t19450 + F::cast_from(0.78066666666666666667e-3_f64) * t104 * t19453 + F::cast_from(0.4684e-2_f64) * t4858 * t19456 - F::cast_from(0.10082625e-4_f64) * t120 * t19459 - F::cast_from(0.672175e-5_f64) * t120 * t19462 + F::cast_from(0.22405833333333333333e-5_f64) * t120 * t19465 - F::cast_from(0.26887e-4_f64) * t4881 * t19468;
    t19471
}
