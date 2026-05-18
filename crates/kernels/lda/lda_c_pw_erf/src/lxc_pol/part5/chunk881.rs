//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 881/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk881<F: Float>(t1084: F, t156: F, t2737: F, t2698: F, t2704: F, t2987: F, t2701: F, t1055: F, t474: F, t39: F, t55: F, t59: F, t87: F) -> (F, F, F, F, F, F) {
    let t8285 = F::new(0.021687161765563047) * t1084 * t156 * t2737;
    let t8286 = t2704 * t2698;
    let t8290 = F::new(38.02486811957057) * t1084 * t156 * t2987;
    let t8291 = t2704 * t2701;
    let t8296 = F::new(1.2842518958703766) * t1084 * t474 * t1055;
    let t8300 = F::new(24.0) * t39 * t55 * t59 * t87;
    (t8285, t8286, t8290, t8291, t8296, t8300)
}
