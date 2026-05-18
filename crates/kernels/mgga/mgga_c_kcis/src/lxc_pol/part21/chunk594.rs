//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 594/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk594<F: Float>(t1036: F, t1670: F, t245: F, t3078: F, t3081: F, t4625: F, t4647: F, t4654: F, t4667: F, t934: F) -> F {
    let t4670 = -t3078 * t4647 / F::new(8.0) + t3081 * t1670 / F::new(4.0) + t1036 * t4625 / F::new(4.0) + t4654 * t934 / F::new(4.0) + t245 * t4667 / F::new(2.0);
    t4670
}
