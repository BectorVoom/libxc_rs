//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 835/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk835<F: Float>(t1772: F, t449: F, t310: F, t448: F, t3086: F, t8414: F, t6548: F, t322: F, t1113: F, t2849: F, t24: F, t3093: F, t1111: F, t195: F, t429: F, t116: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8528 = t1772 * t449;
    let t8529 = t310 * t8528;
    let t8531 = 0.80492236016562572729e-3 * t448 * t8529;
    let t8532 = t3086 * t8414;
    let t8533 = t8532 * t6548;
    let t8534 = t322 * t8533;
    let t8537 = t1113 * t2849;
    let t8538 = t8537 * t6548;
    let t8539 = t322 * t8538;
    let t8542 = t24 * t3093;
    let t8543 = t1111 * t8542;
    let t8545 = t195 * t429;
    let t8546 = t116 * t8545;
    (t8528, t8529, t8531, t8532, t8533, t8534, t8537, t8538, t8539, t8543, t8545, t8546)
}
