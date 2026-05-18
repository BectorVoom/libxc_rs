//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 996/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk996<F: Float>(t1546: F, t22652: F, t4281: F, t7305: F, t22471: F, t22632: F, t22634: F, t22638: F, t22641: F, t22643: F, t22645: F, t22647: F, t22650: F) -> (F, F, F) {
    let t22653 = t1546 * t22652;
    let t22655 = t4281 * t7305;
    let t22657 = -t22471 / F::new(576.0) + t22632 / F::new(16.0) + t22634 / F::new(8.0) - t22638 / F::new(256.0) + t22641 / F::new(192.0) + t22643 / F::new(24.0) - t22645 / F::new(8.0) + t22647 / F::new(3.0) + F::new(3.0) / F::new(128.0) * t22650 - t22653 / F::new(24.0) + t22655 / F::new(256.0);
    (t22653, t22655, t22657)
}
