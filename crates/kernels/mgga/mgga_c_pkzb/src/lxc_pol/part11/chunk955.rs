//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 955/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk955<F: Float>(t10405: F, t10408: F, t10411: F, t10448: F, t10451: F, t10454: F, t10478: F, t1413: F, t1449: F, t3337: F, t4218: F, t430: F, t453: F, t4772: F, t4828: F, t8599: F, t995: F) -> F {
    let t10481 = F::cast_from(0.496875e-1_f64) * t4218 * t3337 - F::cast_from(0.99375e-1_f64) * t8599 * t995 + F::cast_from(0.298125e0_f64) * t4772 * t10405 - F::cast_from(0.99375e-1_f64) * t1413 * t10408 - F::cast_from(0.99375e-1_f64) * t1413 * t10411 + F::cast_from(0.165625e-1_f64) * t430 * t10448 - F::cast_from(0.19875e0_f64) * t4828 * t10451 + F::cast_from(0.1490625e0_f64) * t1449 * t10454 - F::cast_from(0.165625e-1_f64) * t453 * t10478;
    t10481
}
