//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1115;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1116;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta257(t6879: f64, t7170: f64, t6884: f64, t6899: f64, t1323: f64, t2085: f64, t6914: f64, t6921: f64, t6934: f64, t6948: f64, t6917: f64, t6929: f64, t6938: f64, t6941: f64, t6946: f64, t6953: f64, t539: f64, t2086: f64, t225: f64, t1385: f64, t2091: f64, t3887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7171, t7174, t7176, t7179, t7181, t7183, t7185, t7189, t7191) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1115(t6879, t7170, t6884, t6899, t1323, t2085, t6914, t6921, t6934, t6948, t6917, t6929, t6938, t6941, t6946, t6953);
        let (t7192, t7194) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1116(t539, t7191, t2086, t225);
        let t7199 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1117(t1385, t2091, t3887);
    (t7171, t7174, t7176, t7179, t7181, t7183, t7185, t7189, t7191, t7192, t7194, t7199)
}
