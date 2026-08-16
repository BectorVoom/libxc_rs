//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta413<F: Float>(t1354: F, t22765: F, t3858: F, t6945: F, t1339: F, t3851: F, t6936: F, t3856: F, t3788: F, t3793: F, t6604: F, t6919: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22766, t22767, t22768, t22770, t22771, t22773, t22774, t22776, t22777, t22779) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1583::<F>(t1354, t22765, t3858, t6945, t1339, t3851, t6936, t3856, t3788, t3793, t6604, t6919);
    (t22766, t22767, t22768, t22770, t22771, t22773, t22774, t22776, t22777, t22779)
}
