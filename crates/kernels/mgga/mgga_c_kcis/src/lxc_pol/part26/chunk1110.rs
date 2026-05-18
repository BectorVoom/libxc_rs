//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1110/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1110<F: Float>(t28531: F, t303: F, t5732: F, t7914: F, t6176: F, t3961: F, t6140: F) -> (F, F, F, F) {
    let t28532 = t303 * t28531;
    let t28534 = t7914 * t5732;
    let t28535 = t6176 * t28534;
    let t28544 = t3961 * t6140;
    (t28532, t28534, t28535, t28544)
}
