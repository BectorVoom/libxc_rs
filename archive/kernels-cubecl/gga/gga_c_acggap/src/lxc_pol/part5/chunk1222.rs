//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1222/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1222<F: Float>(t3431: F, t5891: F, t1165: F, t3451: F, t4183: F, t5852: F, t3372: F, t6157: F, t13092: F, t5932: F, t17550: F, t5928: F) -> (F, F, F, F, F) {
    let t22349 = t3431 * t5891;
    let t22369 = t3451 * t1165 * t5852 * t4183;
    let t22371 = t3372 * t6157;
    let t22378 = t13092 * t5932;
    let t22380 = t17550 * t5928;
    (t22349, t22369, t22371, t22378, t22380)
}
