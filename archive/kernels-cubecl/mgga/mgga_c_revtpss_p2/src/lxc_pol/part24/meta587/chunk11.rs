//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1835/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1835<F: Float>(t1437: F, t22009: F, t22321: F, t4114: F, t47351: F, t47395: F, t5745: F, t6862: F, t6874: F, t75145: F, t75147: F, t75176: F, t75179: F, t820: F, t86563: F, t91942: F, t92158: F) -> F {
    let t92378 = -F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t91942 + F::cast_from(0.87805989105806821314e-1_f64) * t75145 - F::cast_from(0.87805989105806821314e-1_f64) * t75147 - F::cast_from(0.39512695097613069592e1_f64) * t820 * t22321 * t6874 + F::cast_from(0.23707617058567841754e2_f64) * t5745 * t22009 * t6862 + F::cast_from(0.15611904863012452831e0_f64) * t75176 - F::cast_from(0.1561190486301245283e0_f64) * t75179 - t47351 - F::cast_from(0.11708928647259339623e0_f64) * t86563 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t4114 * t92158 - t47395;
    t92378
}
