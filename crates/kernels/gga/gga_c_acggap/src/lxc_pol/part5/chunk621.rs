//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 621/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk621<F: Float>(t1674: F, t4051: F, t2637: F, t495: F, t694: F, t1390: F, t229: F, t1378: F, t276: F, t40: F, t1388: F, t4027: F, t87: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4052 = t1674 * t4051;
    let t4055 = t694 * t2637 * t495;
    let t4057 = t229 * t1390;
    let t4058 = F::cast_from(8.0_f64) * t4057;
    let t4059 = t1378 * t276;
    let t4060 = t40 * t4059;
    let t4061 = F::cast_from(2.0_f64) * t4060;
    let t4062 = t229 * t1388;
    let t4063 = F::cast_from(8.0_f64) * t4062;
    let t4064 = t4027 * t87;
    (t4052, t4055, t4057, t4058, t4059, t4060, t4061, t4062, t4063, t4064)
}
