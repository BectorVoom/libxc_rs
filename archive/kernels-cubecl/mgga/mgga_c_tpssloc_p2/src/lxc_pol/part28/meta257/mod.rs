//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1115;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1116;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta257<F: Float>(t6879: F, t7170: F, t6884: F, t6899: F, t1323: F, t2085: F, t6914: F, t6921: F, t6934: F, t6948: F, t6917: F, t6929: F, t6938: F, t6941: F, t6946: F, t6953: F, t539: F, t2086: F, t225: F, t1385: F, t2091: F, t3887: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7171, t7174, t7176, t7179, t7181, t7183, t7185, t7189, t7191) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1115::<F>(t6879, t7170, t6884, t6899, t1323, t2085, t6914, t6921, t6934, t6948, t6917, t6929, t6938, t6941, t6946, t6953);
        let (t7192, t7194) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1116::<F>(t539, t7191, t2086, t225);
        let t7199 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1117::<F>(t1385, t2091, t3887);
    (t7171, t7174, t7176, t7179, t7181, t7183, t7185, t7189, t7191, t7192, t7194, t7199)
}
