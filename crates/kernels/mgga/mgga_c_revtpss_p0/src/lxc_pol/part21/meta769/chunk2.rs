//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2725/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2725<F: Float>(t57: F, t10326: F, t10356: F, t11231: F, t13312: F, t14458: F, t1491: F, t2251: F, t2258: F, t4232: F, t4235: F, t4335: F, t49889: F, t606: F, t770: F, t83: F, zeta_threshold: F) -> F {
    let t155 = t57 <= zeta_threshold;
    let t50149 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t4232 * t10356 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4235 * t2251 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1491 * t11231 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t83 * t13312 * t606 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14458 * t2258 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4335 * t10326 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t49889);
    t50149
}
