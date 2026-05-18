//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 569/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk569<F: Float>(t40: F, t4059: F, t1388: F, t229: F, t4027: F, t87: F, t483: F, t803: F, t2898: F, t474: F, t34: F, t817: F) -> (F, F, F, F, F, F) {
    let t4060 = t40 * t4059;
    let t4061 = F::new(2.0) * t4060;
    let t4062 = t229 * t1388;
    let t4063 = F::new(8.0) * t4062;
    let t4064 = t4027 * t87;
    let t4065 = t40 * t4064;
    let t4068 = t483 * t803;
    let t4069 = t40 * t4068;
    let t4070 = t2898 * t474;
    let t4073 = t817 * t34;
    (t4061, t4063, t4065, t4069, t4070, t4073)
}
