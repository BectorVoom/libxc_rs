//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2689/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2689<F: Float>(t1399: F, t14193: F, t46422: F, t47450: F, t47454: F, t47455: F, t48760: F, t49213: F, t49429: F, t49432: F, t49439: F, t49446: F, t49450: F, t5735: F, t5745: F, t5755: F, t9995: F) -> F {
    let t49456 = F::cast_from(0.19514881078765566037e-2_f64) * t49429 - F::cast_from(0.46263278077393568556e-2_f64) * t49432 + F::cast_from(0.11853808529283920877e2_f64) * t5745 * t5735 * t46422 - F::cast_from(0.13878983423218070567e-1_f64) * t47450 + t47454 + F::cast_from(0.15805078039045227836e2_f64) * t49439 * t5735 * t48760 - F::cast_from(0.23707617058567841754e2_f64) * t14193 * t5735 * t9995 - F::cast_from(0.29272321618148349057e-1_f64) * t49446 + F::cast_from(0.11708928647259339623e0_f64) * t49450 - F::cast_from(0.7805952431506226415e-2_f64) * t47455 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t49213 * t1399;
    t49456
}
