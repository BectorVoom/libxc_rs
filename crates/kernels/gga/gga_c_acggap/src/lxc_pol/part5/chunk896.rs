//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 896/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk896<F: Float>(t1181: F, t12816: F, t3361: F, t4643: F, t322: F, t4199: F, t1165: F, t13585: F, t1532: F, t329: F, t56: F, t2029: F, t4258: F, t1008: F, t5237: F, t14283: F, t537: F) -> (F, F, F, F, F, F, F, F) {
    let t16160 = t3361 * t1181 * t4643 * t12816;
    let t16171 = t4199 * t322;
    let t16174 = t13585 * t1165 * t1532 * t16171;
    let t16183 = t329 * t56;
    let t16184 = t16183 * t2029;
    let t16185 = t16184 * t4258;
    let t16191 = t1008 * t5237;
    let t16203 = t14283 * t537;
    (t16160, t16171, t16174, t16183, t16184, t16185, t16191, t16203)
}
