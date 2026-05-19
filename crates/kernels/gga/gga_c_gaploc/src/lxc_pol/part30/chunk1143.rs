//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1143/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1143<F: Float>(t1538: F, t30802: F, t9267: F, t4782: F, t9272: F, t21272: F, t9544: F, t2349: F, t2478: F, t6576: F, t7047: F, t888: F) -> (F, F, F, F, F) {
    let t30805 = F::cast_from(0.38342925953920749676e1_f64) * t9267 * t1538 * t30802;
    let t30808 = F::cast_from(0.23005755572352449806e1_f64) * t9272 * t4782 * t30802;
    let t30809 = t21272 * t9544;
    let t30810 = F::cast_from(0.76685851907841499352e0_f64) * t30809;
    let t30812 = t6576 * t2349 * t2478;
    let t30813 = F::cast_from(0.76685851907841499352e0_f64) * t30812;
    let t30820 = t6576 * t888 * t7047;
    (t30805, t30808, t30810, t30813, t30820)
}
