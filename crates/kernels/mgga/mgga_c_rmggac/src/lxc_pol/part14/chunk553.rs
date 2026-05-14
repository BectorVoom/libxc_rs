//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 553/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk553<F: Float>(t7641: F, t7642: F, t262: F, t7617: F, t2103: F, t22: F, t3826: F, t7635: F, t2115: F, t7638: F, t3819: F, t2118: F, t3851: F, t7199: F, t36: F, t5245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7643 = t7641 * t7642;
    let t7645 = t262 * t7617;
    let t7646 = t2103 * t7645;
    let t7647 = 0.24244143692662525982e-1 * t7646;
    let t7648 = t3826 * t22;
    let t7649 = t7648 * t7635;
    let t7651 = t2115 * t7638;
    let t7652 = 0.4838420607177634088e-3 * t7651;
    let t7653 = t3819 * t22;
    let t7654 = t7653 * t7642;
    let t7656 = t2118 * t7645;
    let t7658 = t3851 * t7199;
    let t7660 = t5245 * t36;
    (t7643, t7645, t7646, t7647, t7648, t7649, t7651, t7652, t7653, t7654, t7656, t7658, t7660)
}
