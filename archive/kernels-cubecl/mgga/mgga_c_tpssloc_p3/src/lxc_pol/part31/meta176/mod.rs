//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta176 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta176<F: Float>(t3297: F, t4724: F, t136: F, t1113: F, t4729: F, t4733: F, t3238: F, t3282: F, t3294: F, t3295: F, t4721: F, t4726: F, t4731: F, t4735: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F) -> (F, F, F, F, F, F, F) {
        let (t4772, t4773, t4775, t4776, t4778, t4779, t4781) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk826::<F>(t3297, t4724, t136, t1113, t4729, t4733, t3238, t3282, t3294, t3295, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770);
    (t4772, t4773, t4775, t4776, t4778, t4779, t4781)
}
