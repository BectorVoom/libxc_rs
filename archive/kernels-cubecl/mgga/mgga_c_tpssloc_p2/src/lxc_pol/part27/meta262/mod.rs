//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1262;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1263;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1264;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1265;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1266;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta262<F: Float>(t1874: F, t7458: F, t1774: F, t1873: F, t109: F, t652: F, t1453: F, t6530: F, t6529: F, t510: F, t1458: F, t1976: F, t1484: F, t25: F, t1915: F, t6554: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7460, t7461) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1262::<F>(t1874, t7458, t1774, t1873);
        let (t7463, t7467) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1263::<F>(t109, t652, t7461, t1453, t6530, t6529);
        let t7468 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1264::<F>(t510, t7467);
        let (t7470, t7472) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1265::<F>(t652, t7468, t1458, t1976);
        let (t7475, t7476, t7479) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1266::<F>(t1484, t25, t1915, t6554);
    (t7460, t7461, t7463, t7467, t7468, t7470, t7472, t7475, t7476, t7479)
}
