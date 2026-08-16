//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1809;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta574<F: Float>(t25055: F, t81591: F, t25217: F, t6547: F, t25060: F, t82209: F, t82211: F, t25192: F, t81651: F, t82074: F, t82259: F, t25054: F, t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87786, t87796, t87804, t87806, t87807, t87835, t87847, t87873) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1809::<F>(t25055, t81591, t25217, t6547, t25060, t82209, t82211, t25192, t81651, t82074, t82259, t25054);
        let (t87898, t87901, t87910, t87915) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1810::<F>(t23030, t25205, t23164, t7479, t82133, t23204, t25216, t6562, t1519, t212, t23171, t6554);
    (t87786, t87796, t87804, t87806, t87807, t87835, t87847, t87873, t87898, t87901, t87910, t87915)
}
