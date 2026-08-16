//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta275 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1245;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1246;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1247;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1248;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta275<F: Float>(t1530: F, t25: F, t1408: F, t1877: F, t1915: F, t2522: F, t6670: F, t7476: F, t7541: F, t1539: F, t6690: F, t6689: F, t1599: F, t1922: F, t1625: F, t225: F, t387: F, t345: F, t1634: F, t6705: F, t6704: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7545, t7552, t7553) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1245::<F>(t1530, t25, t1408, t1877, t1915, t2522, t6670, t7476, t7541, t1539, t6690);
        let t7554 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1246::<F>(t6689, t7553);
        let t7557 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1247::<F>(t1599, t1922);
        let (t7560, t7561) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1248::<F>(t1625, t225, t387);
        let (t7562, t7565, t7566) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1249::<F>(t345, t7561, t1634, t6705, t6704);
    (t7545, t7552, t7553, t7554, t7557, t7560, t7561, t7562, t7565, t7566)
}
