//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 841/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk841<F: Float>(t2697: F, t2703: F, t842: F, t9612: F, t2617: F, t2696: F, t849: F, t820: F, t847: F, t9516: F, t2645: F, t2647: F, t9621: F) -> (F, F, F, F, F, F) {
    let t9988 = t2697 * t2703;
    let t9990 = t9612 * t842;
    let t9993 = t2617 * t2696;
    let t9994 = t9993 * t849;
    let t9997 = t847 * t820 * t9516;
    let t10003 = t2645 * t9621 * t2647;
    (t9988, t9990, t9993, t9994, t9997, t10003)
}
