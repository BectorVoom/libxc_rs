//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1475/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1475<F: Float>(t101: F, t613: F, t655: F, t100: F, t43: F, t658: F, t2349: F, t96: F, t2350: F, t2256: F, t8268: F, t31026: F, t31028: F, t31030: F, t31033: F, t31035: F, t31036: F, t31040: F, t31044: F, t31047: F, t69: F, t8258: F, t8267: F) -> (F, F, F, F, F, F, F) {
    let t31051 = t655 * t613 * t101;
    let t31054 = t43 * t100;
    let t31055 = t31054 * t658;
    let t31058 = t96 * t2349;
    let t31059 = t31058 * t2350;
    let t31062 = t8268 * t2256;
    let t31065 = -t31026 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t31028 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t31030 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t31033 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t31035 * t31036 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8258 * t31040 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8258 * t31044 + t8258 * t31047 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t69 * t31051 + F::cast_from(25.0_f64) / F::cast_from(36.0_f64) * t8267 * t31055 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8267 * t31059 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8267 * t31062;
    (t31051, t31054, t31055, t31058, t31059, t31062, t31065)
}
