//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 739/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk739<F: Float>(t9254: F, t9256: F, t1030: F, t9253: F, t1648: F, t681: F, t1038: F, t5216: F) -> (F, F, F, F, F) {
    let t9257 = t9254 * t9256;
    let t9259 = t1030 * t9253;
    let t9260 = t1648 * t681;
    let t9261 = t1038 * t5216;
    let t9262 = t9260 * t9261;
    (t9257, t9259, t9260, t9261, t9262)
}
