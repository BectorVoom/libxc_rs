//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 927/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk927<F: Float>(t3409: F, t5192: F, t1165: F, t15758: F, t3451: F, t540: F, t3621: F, t4571: F, t4579: F, t4484: F, t3382: F, t4406: F, t1095: F, t1524: F, t384: F, t398: F, t879: F) -> (F, F, F, F, F, F, F) {
    let t17318 = t3409 * t5192;
    let t17327 = t3451 * t1165 * t540 * t15758;
    let t17353 = t3621 * t4571;
    let t17355 = t3621 * t4579;
    let t17357 = t3621 * t4484;
    let t17362 = t3382 * t4406;
    let t17371 = t384 * t398 * t1095 * t1524 * t879;
    (t17318, t17327, t17353, t17355, t17357, t17362, t17371)
}
