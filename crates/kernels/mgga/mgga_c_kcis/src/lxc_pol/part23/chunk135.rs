//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 135/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk135<F: Float>(t509: F, t538: F, t368: F, t545: F, t562: F, t86: F, t552: F) -> (F, F, F) {
    let t565 = t509 * t538;
    let t569 = F::new(0.619125e-2) * t562 * t545 - F::new(0.39796666666666666666e-1) * t86 * t368 * t565;
    let t570 = t569 * t552;
    (t565, t569, t570)
}
