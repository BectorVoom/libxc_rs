//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 707/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk707<F: Float>(t1629: F, t1636: F, t187: F, t4183: F, t4186: F, t4192: F, t4311: F, t4473: F, t4475: F, t4480: F, t4481: F, t4500: F, t633: F, t449: F, t446: F, t1646: F, t2629: F) -> (F, F, F, F) {
    let t4504 = t4183 - t4186 + t4192 - t4311 + t187 * (-t1629 * t4500 - 2.0 * t1636 * t4475 + t4473 * t633 + 2.0 * t4480 * t4481 - t4183 + t4186 - t4192 + t4311);
    let t4505 = t449 * t4504;
    let t4506 = t446 * t4505;
    let t4507 = t4506 / 16.0;
    let t4510 = t2629 * t1646;
    (t4504, t4505, t4507, t4510)
}
