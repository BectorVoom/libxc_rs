//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 553/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk553<F: Float>(t1629: F, t1636: F, t187: F, t4183: F, t4186: F, t4192: F, t4311: F, t4473: F, t4475: F, t4480: F, t4481: F, t4500: F, t633: F) -> F {
    let t4504 = t4183 - t4186 + t4192 - t4311 + t187 * (-t1629 * t4500 - F::new(2.0) * t1636 * t4475 + t4473 * t633 + F::new(2.0) * t4480 * t4481 - t4183 + t4186 - t4192 + t4311);
    t4504
}
