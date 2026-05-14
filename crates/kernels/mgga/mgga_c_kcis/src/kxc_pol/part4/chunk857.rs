//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 857/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk857<F: Float>(t2429: F, t2485: F, t2528: F, t2720: F, t2725: F, t2729: F, t2752: F, t8526: F, t8533: F, t8541: F, t874: F, t8753: F, t8757: F, t8759: F, t8765: F, t8921: F) -> (F, F, F) {
    let t8924 = t2429 * t2485;
    let t8926 = t2429 * t2528;
    let t8930 = 0.2671335375e-1 * t2725 * t8526 + 0.200175e0 * t874 * t8526 + 0.41786499999999999999e-1 * t8533 - 0.41786499999999999999e-1 * t8541 - 0.69644166666666666665e-2 * t8753 - 0.2089325e-1 * t8757 + 0.2671335375e-1 * t8759 * t2729 - 0.13345e0 * t874 * t8765 - 0.66725e-1 * t874 * t8921 + 0.55715333333333333331e-1 * t8924 + 0.27857666666666666666e-1 * t8926 - 0.200175e0 * t2720 * t2752;
    (t8924, t8926, t8930)
}
