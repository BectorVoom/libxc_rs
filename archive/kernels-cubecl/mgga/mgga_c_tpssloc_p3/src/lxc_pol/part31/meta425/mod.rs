//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1548;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1549;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1550;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta425<F: Float>(t22751: F, t6970: F, t3777: F, t6944: F, t3787: F, t59: F, t240: F, t1336: F, t6943: F, t835: F, t1354: F, t6604: F, t6919: F, t6937: F, t6950: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22753, t22756, t22759, t22760, t22761, t22764, t22765) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1548::<F>(t22751, t6970, t3777, t6944, t3787, t59, t240, t1336, t6943, t835);
        let (t22767, t22779) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1549::<F>(t1354, t22765, t6604, t6919);
        let (t22780, t22782, t22783) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1550::<F>(t22779, t6937, t6950, t835, t1336);
    (t22753, t22756, t22759, t22760, t22761, t22764, t22765, t22767, t22779, t22780, t22782, t22783)
}
