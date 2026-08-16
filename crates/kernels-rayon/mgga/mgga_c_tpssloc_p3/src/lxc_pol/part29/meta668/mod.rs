//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2230;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2231;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2232;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2233;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta668(t1831: f64, t80866: f64, t131: f64, t6931: f64, t9537: f64, t26322: f64, t80855: f64, t236: f64, t26318: f64, t91005: f64, t22782: f64, t5234: f64, t1369: f64, t26257: f64, t3876: f64, t80849: f64, t7712: f64, t80939: f64, t22683: f64, t26285: f64, t6546: f64, t16148: f64, t221: f64, t26284: f64, t16153: f64, t26289: f64, t6604: f64, t80887: f64, t91133: f64, t91136: f64, t91138: f64, t91141: f64, t91144: f64, t91145: f64, t91147: f64, t16217: f64, t6952: f64, t1827: f64, t80910: f64, t22756: f64, t5289: f64, t16208: f64, t6945: f64, t16060: f64, t6951: f64, t1878: f64, t80730: f64, t16215: f64, t80893: f64, t1361: f64, t6925: f64, t6976: f64, t22828: f64, t26243: f64, t26271: f64, t80779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91149, t91155, t91159, t91160) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2230(t1831, t80866, t131, t6931, t9537, t26322, t80855, t236, t26318, t91005, t22782, t5234);
        let (t91162, t91163, t91165, t91167, t91171, t91173) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2231(t1369, t91160, t26257, t3876, t1831, t80849, t7712, t80939, t22683, t26285, t6546, t16148, t221, t26284);
        let t91181 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2232(t16153, t221, t26284, t26289, t6604, t80887, t91133, t91136, t91138, t91141, t91144, t91145, t91147, t91149, t91155, t91159, t91162, t91163, t91165, t91167, t91171, t91173);
        let (t91183, t91185, t91187, t91189, t91192, t91194) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2233(t16217, t6952, t1827, t80910, t22756, t5289, t16208, t6945, t16060, t6951, t1369, t1878, t80730);
        let (t91196, t91200, t91204, t91206) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2234(t16215, t221, t91194, t6604, t80893, t1361, t6925, t6976, t22828, t26243, t26271, t80779);
    (t91181, t91183, t91185, t91187, t91189, t91192, t91196, t91200, t91204, t91206)
}
