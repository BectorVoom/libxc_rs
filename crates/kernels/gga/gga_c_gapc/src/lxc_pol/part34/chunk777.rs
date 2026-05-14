//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 777/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk777<F: Float>(t7204: F, t9645: F, t2706: F, t3103: F, t3397: F, t7073: F, t8673: F, t6188: F, t8676: F, t2664: F, t9504: F, t3127: F, t3363: F, t3132: F, t7294: F, t7259: F, t8624: F) -> (F, F, F, F, F, F, F, F) {
    let t9793 = t7204 * t9645;
    let t9795 = t2706 * t3103;
    let t9796 = t9795 * t3397;
    let t9798 = t7073 * t8673;
    let t9799 = t8676 * t6188;
    let t9800 = t9798 * t9799;
    let t9802 = t9504 * t2664;
    let t9804 = t3363 * t3127;
    let t9805 = t9804 * t2664;
    let t9807 = t7294 * t3132;
    let t9808 = t9807 * t2664;
    let t9810 = t7259 * t8624;
    (t9793, t9796, t9799, t9800, t9802, t9805, t9808, t9810)
}
