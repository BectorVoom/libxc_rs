//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2612/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2612<F: Float>(t11797: F, t5005: F, t1174: F, t5045: F, t698: F, t3540: F, t4966: F, t11647: F, t1744: F, t11697: F, t15469: F, t3577: F) -> (F, F, F, F, F) {
    let t53267 = t5005 * t11797;
    let t53270 = t1174 * t698 * t5045;
    let t53272 = t4966 * t3540;
    let t53274 = t1744 * t11647;
    let t53287 = t3577 * t11697 * t15469;
    (t53267, t53270, t53272, t53274, t53287)
}
