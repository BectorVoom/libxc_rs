//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1693;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta498<F: Float>(t22845: F, t28073: F, t1998: F, t236: F, t6347: F, t6926: F, t6375: F, t6916: F, t22761: F, t6390: F, t2002: F, t6378: F, t559: F, t6422: F, t6945: F, t6427: F, t6952: F, t6431: F, t1831: F, t26257: F, t1799: F, t1824: F, t550: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28074, t28077, t28078, t28080, t28085, t28088) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1693::<F>(t22845, t28073, t1998, t236, t6347, t6926, t6375, t6916, t22761, t6390, t2002, t6378);
        let (t28089, t28091, t28093, t28095, t28097, t28100) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1694::<F>(t28088, t559, t6422, t6945, t6427, t6952, t6431, t1831, t26257, t1799, t1824, t550);
    (t28074, t28077, t28078, t28080, t28085, t28088, t28089, t28091, t28093, t28095, t28097, t28100)
}
