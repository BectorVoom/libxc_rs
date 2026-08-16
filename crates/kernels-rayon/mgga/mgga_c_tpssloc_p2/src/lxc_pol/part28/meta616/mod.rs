//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1932;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta616(t26301: f64, t80853: f64, t80855: f64, t22788: f64, t5314: f64, t16333: f64, t6952: f64, t1831: f64, t80866: f64, t131: f64, t6931: f64, t9537: f64, t26322: f64, t236: f64, t26318: f64, t91005: f64, t22782: f64, t5234: f64, t1369: f64, t26257: f64, t3876: f64, t80849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91143, t91145, t91147, t91149, t91152) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1932(t26301, t80853, t80855, t22788, t5314, t16333, t6952, t1831, t80866, t131, t6931, t9537);
        let (t91154, t91158, t91161, t91163, t91165) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1933(t26322, t80855, t91152, t236, t26318, t91005, t22782, t5234, t1369, t26257, t3876, t1831, t80849);
    (t91143, t91145, t91147, t91149, t91154, t91158, t91161, t91163, t91165)
}
