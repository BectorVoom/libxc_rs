//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 748/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk748<F: Float>(t6188: F, t8676: F, t9798: F, t2664: F, t9504: F, t3127: F, t3363: F, t3132: F, t7294: F, t7259: F, t8624: F, t7325: F, t2660: F, t7330: F, t7335: F, t1077: F, t2713: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9799 = t8676 * t6188;
    let t9800 = t9798 * t9799;
    let t9802 = t9504 * t2664;
    let t9804 = t3363 * t3127;
    let t9805 = t9804 * t2664;
    let t9807 = t7294 * t3132;
    let t9808 = t9807 * t2664;
    let t9810 = t7259 * t8624;
    let t9811 = t9810 * t7325;
    let t9815 = t2660 * t8624;
    let t9816 = t9815 * t7330;
    let t9818 = t9810 * t7335;
    let t9820 = t1077 * t2713;
    (t9799, t9800, t9802, t9805, t9808, t9811, t9816, t9818, t9820)
}
