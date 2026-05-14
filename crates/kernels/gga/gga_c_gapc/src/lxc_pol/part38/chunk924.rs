//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 924/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk924<F: Float>(t5056: F, t172: F, t6: F, t674: F, t1672: F, t3074: F, t4: F, t5972: F, t1908: F, t505: F, t647: F, t8715: F, t2999: F, t5216: F, t1648: F, t3005: F) -> (F, F, F, F, F, F) {
    let t27622 = t5056 * M_PI;
    let t27624 = t6 * t674 * t172;
    let t27658 = t1672 * t3074 * t5972 * t4;
    let t27754 = t505 * t1908 * t647 * t8715;
    let t27867 = t2999 * t5216;
    let t27868 = t1648 * t3005 * t27867;
    (t27622, t27624, t27658, t27754, t27867, t27868)
}
