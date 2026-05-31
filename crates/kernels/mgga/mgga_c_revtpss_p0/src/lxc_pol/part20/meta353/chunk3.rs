//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1287/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1287<F: Float>(t45: F, t57: F, t10326: F, t10472: F, t2251: F, t2258: F, t2299: F, t39443: F, t39449: F, t39457: F, t633: F, t766: F, t80: F, t10481: F, t2306: F, t637: F, t770: F, t83: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t39461 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t2299 * t39443 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t633 * t2251 * t2258 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t80 * t39449 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t10472 * t10326 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t39457);
    let t39474 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t2306 * t39443 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t637 * t2251 * t2258 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t83 * t39449 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t10481 * t10326 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t39457);
    (t39461, t39474)
}
