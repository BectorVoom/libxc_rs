//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta527<F: Float>(t20038: F, t225: F, t20032: F, t20040: F, t19635: F, t20048: F, t1351: F, t6414: F, t6387: F, t6330: F, t12250: F, t1834: F, t5286: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t56434, t56580, t56596, t56607, t56640, t56812, t57091, t57172, t57342, t57499) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1741::<F>(t20038, t225, t20032, t20040, t19635, t20048, t1351, t6414, t6387, t6330, t12250, t1834, t5286);
    (t56434, t56580, t56596, t56607, t56640, t56812, t57091, t57172, t57342, t57499)
}
