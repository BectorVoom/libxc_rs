//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1246;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1247;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta274(t1530: f64, t25: f64, t1408: f64, t1877: f64, t1915: f64, t2522: f64, t6670: f64, t7476: f64, t7541: f64, t1409: f64, t3: f64, t1597: f64, t343: f64, t1484: f64, t202: f64, t7540: f64, t193: f64, t870: f64, t28: f64, t1649: f64, t1873: f64, t4028: f64, t1458: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7545, t7552, t7573, t7577) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1246(t1530, t25, t1408, t1877, t1915, t2522, t6670, t7476, t7541, t1409, t3, t1597, t343);
        let (t7637, t7642, t7649, t7656, t7663) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1247(t1484, t1915, t202, t7540, t1530, t1877, t193, t2522, t6670, t870, t28, t1649, t7541);
        let (t7675, t7676) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1248(t1873, t4028, t1458, t88);
    (t7545, t7552, t7573, t7577, t7637, t7642, t7649, t7656, t7663, t7675, t7676)
}
