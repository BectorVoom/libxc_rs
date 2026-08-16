//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1854;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1855;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1856;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1857;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1858;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1859;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta521(t1873: f64, t26114: f64, t4072: f64, t88: f64, t6534: f64, t7676: f64, t2314: f64, t7467: f64, t5113: f64, t1453: f64, t22470: f64, t666: f64, t109: f64, t22473: f64, t4067: f64, t6530: f64, t22469: f64, t22471: f64, t1268: f64, t1458: f64, t22461: f64, t24999: f64, t26098: f64, t26103: f64, t26109: f64, t26111: f64, t26113: f64, t6517: f64, t671: f64, t12725: f64, t1874: f64, t510: f64, t652: f64, t7000: f64, t7685: f64, t6876: f64, t7688: f64, t6999: f64, t7753: f64, t1983: f64, t6880: f64, t1266: f64, t1976: f64, t1980: f64, t26002: f64, t26005: f64, t26006: f64, t4026: f64, t5361: f64, t574: f64, t7451: f64, t7754: f64, t1982: f64, t8944: f64, t12461: f64, t2018: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26116, t26117, t26119, t26121, t26123, t26125, t26127, t26129) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1854(t1873, t26114, t4072, t88, t6534, t7676, t2314, t7467, t5113, t1453, t22470, t666);
        let t26135 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1855(t109, t22473, t26129, t4067, t6530, t22469, t22471, t26127);
        let t26138 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1856(t1268, t26135, t1458, t22461, t24999, t26098, t26103, t26109, t26111, t26113, t26116, t26119, t26121, t26123, t26125, t4072, t6517, t671);
        let (t26141, t26142, t26144, t26145, t26147, t26149, t26150) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1857(t12725, t1874, t26135, t510, t652, t7000, t7685, t6876, t7688, t6999, t7753, t1983);
        let t26155 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1858(t6880, t7685, t1266, t1976, t1980, t26002, t26005, t26006, t26098, t26138, t26141, t26144, t26145, t26147, t26150, t4026, t510, t5361, t574, t7451);
        let (t26157, t26161) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1859(t6876, t7754, t1982, t8944);
        let t26162 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1860(t12461, t2018);
    (t26117, t26127, t26129, t26135, t26138, t26142, t26149, t26155, t26157, t26161, t26162)
}
