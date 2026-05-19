//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1043/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1043<F: Float>(t1796: F, t3648: F, t603: F, t22497: F, t22562: F, t22578: F, t22581: F, t22593: F, t22681: F, t22683: F, t22685: F, t22687: F, t22690: F, t22694: F) -> (F, F) {
    let t22697 = F::cast_from(0.67471169937307261776e-1_f64) * t1796 * t3648 * t603;
    let t22698 = t22681 - t22683 - t22685 + t22687 - t22690 - t22694 - t22497 + t22562 + t22578 + t22581 - t22593 + t22697;
    (t22697, t22698)
}
