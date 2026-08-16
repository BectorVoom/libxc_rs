//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1476/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1476<F: Float>(t114: F, t31026: F, t31035: F, t31259: F, t31274: F, t31538: F, t31542: F, t31545: F, t31548: F, t31551: F, t8258: F, t8267: F) -> F {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t31555 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t31026 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t31259 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t31274 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t31035 * t31538 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8258 * t31542 + t8258 * t31545 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8267 * t31548 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8267 * t31551);
    t31555
}
