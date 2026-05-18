//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 925/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk925<F: Float>(t888: F, t8920: F, t2429: F, t2485: F, t2528: F, t2720: F, t2725: F, t2729: F, t2752: F, t8526: F, t8533: F, t8541: F, t874: F, t8753: F, t8757: F, t8759: F, t8765: F) -> (F, F, F) {
    let t8921 = t8920 * t888;
    let t8924 = t2429 * t2485;
    let t8926 = t2429 * t2528;
    let t8930 = F::new(0.2671335375e-1) * t2725 * t8526 + F::new(0.200175e0) * t874 * t8526 + F::new(0.41786499999999999999e-1) * t8533 - F::new(0.41786499999999999999e-1) * t8541 - F::new(0.69644166666666666665e-2) * t8753 - F::new(0.2089325e-1) * t8757 + F::new(0.2671335375e-1) * t8759 * t2729 - F::new(0.13345e0) * t874 * t8765 - F::new(0.66725e-1) * t874 * t8921 + F::new(0.55715333333333333331e-1) * t8924 + F::new(0.27857666666666666666e-1) * t8926 - F::new(0.200175e0) * t2720 * t2752;
    (t8924, t8926, t8930)
}
