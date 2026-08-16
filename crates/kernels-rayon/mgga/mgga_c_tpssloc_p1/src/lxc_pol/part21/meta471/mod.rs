//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2050;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta471(t16242: f64, t5248: f64, t5250: f64, t12240: f64, t5249: f64, t3856: f64, t12283: f64, t5303: f64, t1352: f64, t3851: f64, t1340: f64, t16060: f64, t3789: f64, t5234: f64, t3798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16257, t16261, t16265, t16269, t16271, t16275, t16278) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2050(t16242, t5248, t5250, t12240, t5249, t3856, t12283, t5303, t1352, t3851, t1340, t16060);
        let (t16285, t16288) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2051(t3789, t5234, t3798);
    (t16257, t16261, t16265, t16269, t16271, t16275, t16278, t16285, t16288)
}
