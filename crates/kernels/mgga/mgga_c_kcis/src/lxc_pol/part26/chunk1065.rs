//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1065/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1065<F: Float>(t29459: F, t29461: F, t29463: F, t29466: F, t29468: F, t29471: F, t29473: F, t29475: F, t29477: F, t29480: F, t29482: F, t29484: F, t29638: F, t1629: F, t29415: F, t29417: F, t29418: F, t29419: F, t29423: F, t29426: F, t29429: F, t29432: F, t29488: F, t29624: F, t633: F) -> (F, F) {
    let t29651 = 0.20234375e-1 * t29459 + 0.91666666666666666667e0 * t29461 - 0.33333333333333333334e0 * t29463 - 0.9375e-1 * t29466 - 0.1875e0 * t29468 - 0.20833333333333333333e-1 * t29471 - 0.89930555555555555557e-2 * t29473 + 0.9375e-1 * t29475 - 0.26979166666666666667e-1 * t29477 + 0.625e-1 * t29480 - 0.5e0 * t29482 + 0.125e0 * t29484;
    let t29652 = t29638 + t29651;
    let t29654 = -t1629 * t29652 + t29624 * t633 + t29415 - t29417 + t29418 + t29419 + t29423 + t29426 - t29429 - t29432 + t29488;
    (t29652, t29654)
}
