//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2090;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta665(t91137: f64, t26297: f64, t80853: f64, t80855: f64, t26301: f64, t1831: f64, t80866: f64, t131: f64, t6931: f64, t9537: f64, t26322: f64, t236: f64, t26318: f64, t91005: f64, t22782: f64, t5234: f64, t1369: f64, t7712: f64, t80939: f64, t22683: f64, t26285: f64, t6546: f64, t26289: f64, t6604: f64, t80887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91138, t91141, t91144, t91149, t91155, t91158) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2090(t91137, t26297, t80853, t80855, t26301, t1831, t80866, t131, t6931, t9537, t26322, t236, t26318, t91005);
        let (t91159, t91160, t91162, t91167, t91171, t91179) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2091(t91158, t22782, t5234, t1369, t7712, t80939, t22683, t26285, t6546, t26289, t6604, t80887);
    (t91138, t91141, t91144, t91149, t91155, t91159, t91160, t91162, t91167, t91171, t91179)
}
