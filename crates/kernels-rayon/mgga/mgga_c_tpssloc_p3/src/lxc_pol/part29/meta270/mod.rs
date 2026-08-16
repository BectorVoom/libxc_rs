//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1264;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1265;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1266;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta270(t1528: f64, t1912: f64, t259: f64, t4147: f64, t4268: f64, t6549: f64, t6565: f64, t6627: f64, t7481: f64, t7486: f64, t7490: f64, t7492: f64, t7511: f64, t7517: f64, t7538: f64, t855: f64, t870: f64, t1530: f64, t25: f64, t1408: f64, t1877: f64, t1915: f64, t2522: f64, t6670: f64, t7476: f64, t1409: f64, t3: f64, t1484: f64, t202: f64, t193: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7540 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1264(t1528, t1912, t259, t4147, t4268, t6549, t6565, t6627, t7481, t7486, t7490, t7492, t7511, t7517, t7538, t855);
        let t7541 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1265(t7540, t870);
        let (t7545, t7552, t7573, t7634) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1266(t1530, t25, t1408, t1877, t1915, t2522, t6670, t7476, t7541, t1409, t3, t1484);
        let (t7642, t7649, t7650) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1267(t202, t7540, t1530, t1877, t193, t2522, t6670, t7634, t870, t1484, t28, t1915);
    (t7540, t7541, t7545, t7552, t7573, t7634, t7642, t7649, t7650)
}
