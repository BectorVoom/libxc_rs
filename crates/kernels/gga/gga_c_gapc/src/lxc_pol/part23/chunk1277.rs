//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1277/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1277<F: Float>(t24625: F, t3643: F, t3734: F, t11270: F, t11648: F, t24398: F, t10328: F, t11688: F, t23132: F, t24081: F, t17874: F, t35469: F) -> (F, F, F, F, F) {
    let t35759 = t3643 * t24625 * t3734;
    let t35762 = t11270 * t24398 * t11648;
    let t35764 = t10328 * t11688;
    let t35766 = t24081 * t23132;
    let t35768 = t35766 * t35469 * t17874;
    (t35759, t35762, t35764, t35766, t35768)
}
