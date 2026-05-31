//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 901/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk901<F: Float>(t1413: F, t1449: F, t2481: F, t2484: F, t2507: F, t2510: F, t430: F, t453: F, t459: F, t4769: F, t4772: F, t4828: F, t6631: F, t6634: F, t6639: F, t6642: F, t6645: F, t6692: F, t6700: F, t6703: F, t6706: F, t6709: F, t6712: F, t6747: F, t987: F, t995: F) -> F {
    let t6750 = F::cast_from(0.165625e-1_f64) * t6631 * t987 - F::cast_from(0.6625e-1_f64) * t6634 * t2484 + F::cast_from(0.33125e-1_f64) * t2481 * t2507 + F::cast_from(0.99375e-1_f64) * t4772 * t6639 - F::cast_from(0.6625e-1_f64) * t1413 * t6642 - F::cast_from(0.33125e-1_f64) * t1413 * t6645 + F::cast_from(0.165625e-1_f64) * t430 * t6692 - F::cast_from(0.33125e-1_f64) * t4769 * t995 + F::cast_from(0.19875e0_f64) * t4772 * t2510 * t459 - F::cast_from(0.6625e-1_f64) * t1413 * t6700 - F::cast_from(0.33125e-1_f64) * t1413 * t6703 - F::cast_from(0.19875e0_f64) * t4828 * t6706 + F::cast_from(0.99375e-1_f64) * t1449 * t6709 + F::cast_from(0.496875e-1_f64) * t1449 * t6712 - F::cast_from(0.165625e-1_f64) * t453 * t6747;
    t6750
}
