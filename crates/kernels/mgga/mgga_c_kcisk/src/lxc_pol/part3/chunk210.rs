//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 210/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk210<F: Float>(t163: F, t80: F, t81: F, t867: F, t869: F, t874: F, t88: F) -> (F, F, F) {
    let t877 = t80 * t81 * t163;
    let t879 = -F::new(0.632975e0) * t867 - F::new(0.29896666666666666667e0) * t869 - F::new(0.1023875e0) * t874 - F::new(0.82156666666666666667e-1) * t877;
    let t880 = F::new(1.0) / t88;
    (t877, t879, t880)
}
