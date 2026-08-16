//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta668 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2230;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2231;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2232;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2233;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta668<F: Float>(t1831: F, t80866: F, t131: F, t6931: F, t9537: F, t26322: F, t80855: F, t236: F, t26318: F, t91005: F, t22782: F, t5234: F, t1369: F, t26257: F, t3876: F, t80849: F, t7712: F, t80939: F, t22683: F, t26285: F, t6546: F, t16148: F, t221: F, t26284: F, t16153: F, t26289: F, t6604: F, t80887: F, t91133: F, t91136: F, t91138: F, t91141: F, t91144: F, t91145: F, t91147: F, t16217: F, t6952: F, t1827: F, t80910: F, t22756: F, t5289: F, t16208: F, t6945: F, t16060: F, t6951: F, t1878: F, t80730: F, t16215: F, t80893: F, t1361: F, t6925: F, t6976: F, t22828: F, t26243: F, t26271: F, t80779: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91149, t91155, t91159, t91160) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2230::<F>(t1831, t80866, t131, t6931, t9537, t26322, t80855, t236, t26318, t91005, t22782, t5234);
        let (t91162, t91163, t91165, t91167, t91171, t91173) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2231::<F>(t1369, t91160, t26257, t3876, t1831, t80849, t7712, t80939, t22683, t26285, t6546, t16148, t221, t26284);
        let t91181 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2232::<F>(t16153, t221, t26284, t26289, t6604, t80887, t91133, t91136, t91138, t91141, t91144, t91145, t91147, t91149, t91155, t91159, t91162, t91163, t91165, t91167, t91171, t91173);
        let (t91183, t91185, t91187, t91189, t91192, t91194) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2233::<F>(t16217, t6952, t1827, t80910, t22756, t5289, t16208, t6945, t16060, t6951, t1369, t1878, t80730);
        let (t91196, t91200, t91204, t91206) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2234::<F>(t16215, t221, t91194, t6604, t80893, t1361, t6925, t6976, t22828, t26243, t26271, t80779);
    (t91181, t91183, t91185, t91187, t91189, t91192, t91196, t91200, t91204, t91206)
}
