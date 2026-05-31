//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1130/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1130<F: Float>(t1424: F, t454: F, t34: F, t4794: F, t38: F, t4810: F, t19378: F, t19381: F, t19384: F, t19387: F, t19390: F, t19397: F, t19400: F, t19403: F, t19410: F, t19435: F, t19439: F, t6655: F, t6659: F, t6668: F, t6723: F, t6738: F) -> F {
    let t19520 = t454 * t1424;
    let t19523 = t34 * t4794;
    let t19530 = t38 * t4810;
    let t19539 = F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t454 * t6655 + F::cast_from(25.0_f64) * t454 * t6668 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t34 * t19435 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t34 * t19439 - F::cast_from(10.0_f64) * t6723 * t19387 + F::cast_from(10.0_f64) * t6738 * t19390 - F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t19520 * t6659 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19523 * t19397 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19523 * t19400 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t6723 * t19403 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19530 * t19378 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19530 * t19381 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t6738 * t19384 - F::cast_from(10.0_f64) * t34 * t19410;
    t19539
}
