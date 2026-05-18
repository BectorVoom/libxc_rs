//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 687/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk687<F: Float>(t7615: F, t7618: F, t7620: F, t7622: F, t7625: F, t7628: F) -> F {
    let t7669 = F::new(0.1875e0) * t7615 - F::new(0.1875e0) * t7618 - F::new(0.375e0) * t7620 - F::new(0.809375e-1) * t7622 + F::new(0.809375e-1) * t7625 + F::new(0.32375e0) * t7628;
    t7669
}
