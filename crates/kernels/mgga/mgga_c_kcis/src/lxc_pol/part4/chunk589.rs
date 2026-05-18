//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 589/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk589<F: Float>(t1036: F, t245: F, t2944: F, t2952: F, t3078: F, t3081: F, t3093: F, t934: F) -> F {
    let t3096 = -t3078 * t2944 / F::new(8.0) + t3081 * t934 / F::new(2.0) + t1036 * t2952 / F::new(4.0) + t245 * t3093 / F::new(2.0);
    t3096
}
