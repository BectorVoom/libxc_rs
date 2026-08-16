//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1246;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1247;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta274<F: Float>(t1530: F, t25: F, t1408: F, t1877: F, t1915: F, t2522: F, t6670: F, t7476: F, t7541: F, t1409: F, t3: F, t1597: F, t343: F, t1484: F, t202: F, t7540: F, t193: F, t870: F, t28: F, t1649: F, t1873: F, t4028: F, t1458: F, t88: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7545, t7552, t7573, t7577) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1246::<F>(t1530, t25, t1408, t1877, t1915, t2522, t6670, t7476, t7541, t1409, t3, t1597, t343);
        let (t7637, t7642, t7649, t7656, t7663) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1247::<F>(t1484, t1915, t202, t7540, t1530, t1877, t193, t2522, t6670, t870, t28, t1649, t7541);
        let (t7675, t7676) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1248::<F>(t1873, t4028, t1458, t88);
    (t7545, t7552, t7573, t7577, t7637, t7642, t7649, t7656, t7663, t7675, t7676)
}
