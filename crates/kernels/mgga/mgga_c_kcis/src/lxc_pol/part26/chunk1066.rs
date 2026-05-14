//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1066/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1066<F: Float>(t187: F, t29413: F, t29415: F, t29417: F, t29418: F, t29419: F, t29421: F, t29423: F, t29426: F, t29429: F, t29432: F, t29488: F, t29508: F, t29654: F, t449: F, t446: F) -> (F, F) {
    let t29657 = t29413 - t29415 + t29417 - t29418 - t29419 + t29421 - t29423 - t29426 + t29429 + t29432 - t29488 + t187 * (t29508 + t29654);
    let t29658 = t449 * t29657;
    let t29659 = t446 * t29658;
    (t29657, t29659)
}
