//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2713/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2713<F: Float>(t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39490: F, t39496: F, t56202: F, t56203: F, t56207: F, t56208: F, t56219: F, t56279: F, t56298: F, t56299: F, t56351: F, t56362: F, t56363: F) -> F {
    let t57197 = t39463 - t39468 + t56202 - t56203 + t56207 + t56208 - t39472 - t39476 - t56219 - t56279 + t56298 + t56299 + t56351 + t39483 - t56362 - t39490 + t56363 - t39496;
    t57197
}
