//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1445;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta290(t68: f64, t9971: f64, t226: f64, t1519: f64, t2627: f64, t4265: f64, t814: f64, t4280: f64, t808: f64, t225: f64, t4149: f64, t4351: f64, t892: f64, t1543: f64, t2841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13396, t13397, t13416, t13433, t13453, t13463, t13515) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1445(t68, t9971, t226, t1519, t2627, t4265, t814, t4280, t808, t225, t4149, t4351, t892);
        let t13520 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1446(t1543, t2841);
    (t13396, t13397, t13416, t13433, t13453, t13463, t13515, t13520)
}
