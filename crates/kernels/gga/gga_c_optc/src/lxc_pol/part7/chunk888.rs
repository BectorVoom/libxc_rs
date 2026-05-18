//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 888/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk888<F: Float>(t1111: F, t8542: F, t195: F, t429: F, t116: F, t428: F, t1093: F, t2916: F, t3054: F, t1102: F, t2917: F) -> (F, F, F, F, F, F, F) {
    let t8543 = t1111 * t8542;
    let t8545 = t195 * t429;
    let t8546 = t116 * t8545;
    let t8548 = F::new(5.0) / F::new(1296.0) * t428 * t8546;
    let t8549 = t2916 * t1093;
    let t8550 = t8549 * t3054;
    let t8552 = F::new(0.35089340384731224426e1) * t1102 * t8550;
    let t8553 = t2917 * t1093;
    (t8543, t8545, t8548, t8549, t8550, t8552, t8553)
}
