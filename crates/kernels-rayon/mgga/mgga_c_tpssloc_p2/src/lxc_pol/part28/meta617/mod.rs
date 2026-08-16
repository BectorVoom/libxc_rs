//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1934;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta617(t7712: f64, t80939: f64, t22683: f64, t26285: f64, t6546: f64, t16148: f64, t221: f64, t26284: f64, t16153: f64, t26289: f64, t6604: f64, t80887: f64, t16217: f64, t6952: f64, t1827: f64, t80910: f64, t22756: f64, t5289: f64, t16208: f64, t6945: f64, t16060: f64, t6951: f64, t1369: f64, t1878: f64, t80730: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91167, t91170, t91173, t91176, t91179) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1934(t7712, t80939, t22683, t26285, t6546, t16148, t221, t26284, t16153, t26289, t6604, t80887);
        let (t91183, t91185, t91187, t91189, t91192, t91194) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1935(t16217, t6952, t1827, t80910, t22756, t5289, t16208, t6945, t16060, t6951, t1369, t1878, t80730);
    (t91167, t91170, t91173, t91176, t91179, t91183, t91185, t91187, t91189, t91192, t91194)
}
