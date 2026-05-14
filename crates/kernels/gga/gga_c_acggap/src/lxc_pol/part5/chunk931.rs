//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 931/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk931<F: Float>(t14283: F, t542: F, t4886: F, t997: F, t1576: F, t3237: F, t13502: F, t537: F, t5237: F, t1032: F, t4557: F, t1008: F, t5267: F, t1441: F, t3670: F, t14053: F, t1541: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17501 = t14283 * t542;
    let t17503 = t997 * t4886;
    let t17505 = t3237 * t1576;
    let t17507 = t13502 * t537;
    let t17509 = t997 * t5237;
    let t17511 = t1032 * t4557;
    let t17513 = t1008 * t5267;
    let t17521 = t997 * t5267;
    let t17528 = t3670 * t1441;
    let t17530 = t14053 * t1541;
    (t17501, t17503, t17505, t17507, t17509, t17511, t17513, t17521, t17528, t17530)
}
