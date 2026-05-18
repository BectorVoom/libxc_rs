//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1030/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1030<F: Float>(t1008: F, t4849: F, t4853: F, t4878: F, t1181: F, t3290: F, t3391: F, t6337: F, t3409: F, t4406: F, t1165: F, t12991: F, t3355: F, t540: F) -> (F, F, F, F, F, F) {
    let t17619 = t1008 * t4849;
    let t17621 = t1008 * t4853;
    let t17623 = t1008 * t4878;
    let t17627 = t3391 * t1181 * t6337 * t3290;
    let t17631 = t3409 * t4406;
    let t17635 = t12991 * t1165 * t540 * t3355;
    (t17619, t17621, t17623, t17627, t17631, t17635)
}
