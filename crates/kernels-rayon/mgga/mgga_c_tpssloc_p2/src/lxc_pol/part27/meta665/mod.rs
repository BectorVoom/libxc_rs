//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2335;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2336;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2337;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta665(t1354: f64, t91278: f64, t1827: f64, t80991: f64, t22765: f64, t5289: f64, t22764: f64, t5234: f64, t26298: f64, t80958: f64, t1307: f64, t1339: f64, t22827: f64, t5287: f64, t54068: f64, t550: f64, t22779: f64, t26319: f64, t80837: f64, t80843: f64, t80848: f64, t80857: f64, t80859: f64, t91261: f64, t91263: f64, t91268: f64, t91272: f64, t91276: f64, t1358: f64, t26248: f64, t3862: f64, t7715: f64, t22705: f64, t22852: f64, t236: f64, t5286: f64, t26245: f64, t80791: f64, t80867: f64, t22788: f64, t5310: f64, t16150: f64, t6952: f64, t16155: f64, t26271: f64, t80836: f64, t1361: f64, t22690: f64, t22792: f64, t5187: f64, t16148: f64, t26288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91279, t91282, t91284, t91287, t91290, t91294) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2335(t1354, t91278, t1827, t80991, t22765, t5289, t22764, t5234, t26298, t80958, t1307, t1339, t22827, t5287);
        let t91302 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2336(t1339, t22827, t54068, t550, t22779, t26319, t80837, t80843, t80848, t80857, t80859, t91261, t91263, t91268, t91272, t91276, t91279, t91282, t91284, t91287, t91290, t91294);
        let (t91304, t91305, t91311, t91312, t91314) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2337(t1358, t26248, t3862, t7715, t22705, t22852, t236, t5286, t550, t26245, t80791, t80867);
        let (t91317, t91319, t91321, t91323, t91328, t91330) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2338(t22788, t5310, t16150, t6952, t16155, t26271, t80836, t1361, t22690, t22792, t5187, t16148, t26288);
    (t91302, t91304, t91305, t91311, t91312, t91314, t91317, t91319, t91321, t91323, t91328, t91330)
}
