//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1154/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1154<F: Float>(t1181: F, t1532: F, t1753: F, t3451: F, t864: F, t14050: F, t5737: F, t1165: F, t14187: F, t301: F, t3457: F, t5852: F) -> (F, F, F) {
    let t20826 = t3451 * t1181 * t1532 * t1753 * t864;
    let t20830 = t14050 * t5737;
    let t20836 = t14187 * t1165 * t5852 * t3457 * t301;
    (t20826, t20830, t20836)
}
