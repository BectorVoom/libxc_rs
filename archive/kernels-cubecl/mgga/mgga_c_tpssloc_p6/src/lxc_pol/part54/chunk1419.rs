//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1419/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1419<F: Float>(t22635: F, t26331: F, t31549: F, t5308: F, t1985: F, t26193: F, t31607: F, t115519: F, t120309: F, t120312: F, t120313: F, t120316: F, t120321: F, t120324: F, t1375: F, t1843: F, t2016: F, t2091: F, t22670: F, t26471: F, t33294: F, t3882: F, t3887: F, t7937: F, t93341: F) -> F {
    let t122227 = t26331 * t22635 * t31549 * t5308;
    let t122235 = t1985 * t26193 * t31607;
    let t122240 = -F::cast_from(0.49348022005446793095e-1_f64) * t122227 - t3882 * t33294 + F::cast_from(2.0_f64) * t1375 * t3887 * t2091 * t26471 + t120309 - t120312 + t120313 - t120316 - F::cast_from(0.82246703342411321825e-2_f64) * t122235 + t120321 - t93341 * t2016 - t120324 - t115519 * t1843 - t22670 * t7937;
    t122240
}
