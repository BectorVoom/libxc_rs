//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1388/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1388<F: Float>(t26663: F, t26666: F, t26668: F, t26670: F, t91785: F, t91786: F, t95277: F, t95278: F, t95279: F, t95280: F, t95281: F, t97609: F) -> F {
    let tv4rho3sigma3 = t95277 + t26663 - t26666 - t91785 - t95278 - t95279 + t91786 + t26668 + t26670 - t95280 - t95281 + t97609;
    tv4rho3sigma3
}
