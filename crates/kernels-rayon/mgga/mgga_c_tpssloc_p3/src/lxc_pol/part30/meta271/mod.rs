//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1230;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1231;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1232;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1233;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta271(t1874: f64, t7458: f64, t1774: f64, t1873: f64, t109: f64, t652: f64, t1453: f64, t6530: f64, t6529: f64, t510: f64, t1458: f64, t1976: f64, t1484: f64, t25: f64, t1915: f64, t6554: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7460, t7461) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1230(t1874, t7458, t1774, t1873);
        let (t7463, t7467) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1231(t109, t652, t7461, t1453, t6530, t6529);
        let t7468 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1232(t510, t7467);
        let (t7470, t7472) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1233(t652, t7468, t1458, t1976);
        let (t7475, t7476, t7479) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1234(t1484, t25, t1915, t6554);
    (t7460, t7461, t7463, t7467, t7468, t7470, t7472, t7475, t7476, t7479)
}
