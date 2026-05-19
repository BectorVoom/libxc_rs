//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 859/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk859<F: Float>(t3525: F, t683: F, t1899: F, t1084: F, t2782: F, t1855: F, t3554: F, t5776: F, t3551: F, t1901: F, t3550: F, t2786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9216 = t3525 * t683;
    let t9218 = F::new(6.0) * t1899 * t9216;
    let t9219 = t1084 * t2782;
    let t9221 = F::new(4.0) * t1855 * t9219;
    let t9222 = t3554 * t683;
    let t9224 = F::cast_from(0.96491876992155210402e2_f64) * t5776 * t9222;
    let t9225 = t3551 * t683;
    let t9227 = F::new(2.0) * t1855 * t9225;
    let t9228 = t3550 * t1901;
    let t9229 = t9228 * t683;
    let t9231 = F::cast_from(0.16081979498692535067e2_f64) * t1899 * t9229;
    let t9232 = t2786 * t2782;
    (t9216, t9218, t9219, t9221, t9222, t9224, t9225, t9227, t9228, t9229, t9231, t9232)
}
