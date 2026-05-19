//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 997/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk997<F: Float>(t17428: F, t17431: F, t17434: F, t17437: F, t17439: F, t17441: F, t17444: F, t17447: F, t17451: F, t17455: F, t17458: F, t17461: F, t17465: F, t17468: F, t17472: F, t17475: F, t17478: F, t17481: F) -> F {
    let t18331 = F::cast_from(0.23981481481481481481e-1_f64) * t17428 - F::new(0.125e0) * t17431 + F::cast_from(0.71944444444444444444e-1_f64) * t17434 - F::cast_from(0.26979166666666666666e-1_f64) * t17437 + F::new(0.20234375e-1) * t17439 - F::cast_from(0.26979166666666666666e-1_f64) * t17441 - F::cast_from(0.89930555555555555554e-2_f64) * t17444 + F::cast_from(0.26979166666666666666e-1_f64) * t17447 - F::new(0.1875e0) * t17451 + F::cast_from(0.89930555555555555554e-2_f64) * t17455 + F::cast_from(0.13489583333333333333e-1_f64) * t17458 - F::new(0.625e-1) * t17461 + F::new(0.60703125e-1) * t17465 + F::cast_from(0.13489583333333333333e-1_f64) * t17468 + F::cast_from(0.29976851851851851851e-2_f64) * t17472 + F::cast_from(0.33333333333333333334e0_f64) * t17475 + F::new(0.1875e0) * t17478 + F::new(0.25e0) * t17481;
    t18331
}
