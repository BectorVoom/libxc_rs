//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1215;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta204(t1215: f64, t3508: f64, t4977: f64, t4582: f64, t1216: f64, t3242: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t4978 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1215(t1215, t3508);
        let (t4979, t4980, t4983, t4984, t4987) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1216(t4977, t4978, t4582, t1216, t3242, t3584);
    (t4978, t4979, t4980, t4983, t4984, t4987)
}
