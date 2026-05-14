//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 506/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk506<F: Float>(t3794: F, t3795: F, t5469: F, t5472: F, t5475: F, t5479: F) -> (F,) {
    let t5481 = t3794 + t3795 / 9.0 + t5469 / 9.0 - 2.0 / 9.0 * t5472 + 2.0 / 3.0 * t5475 + 2.0 / 3.0 * t5479;
    (t5481,)
}
