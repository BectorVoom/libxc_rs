//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 567/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk567<F: Float>(t2919: F, t2947: F, t4612: F, t4615: F, t4618: F, t4623: F) -> (F,) {
    let t4625 = t2947 + t2919 / 9.0 + t4612 / 9.0 - 2.0 / 9.0 * t4615 + 2.0 / 3.0 * t4618 - 2.0 / 3.0 * t4623;
    (t4625,)
}
