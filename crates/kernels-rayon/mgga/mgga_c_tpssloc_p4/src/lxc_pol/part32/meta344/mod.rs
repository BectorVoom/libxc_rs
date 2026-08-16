//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1383;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta344(t2696: f64, t4166: f64, t849: f64, t13176: f64, t842: f64, t1516: f64, t9601: f64, t1509: f64, t852: f64, t252: f64, t4233: f64, t68: f64, t9971: f64, t226: f64, t4265: f64, t814: f64, t225: f64, t4149: f64, t4351: f64, t892: f64, t1543: f64, t2841: f64, t4389: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13360, t13362, t13365, t13368, t13380, t13384, t13396) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1383(t2696, t4166, t849, t13176, t842, t1516, t9601, t1509, t852, t252, t4233, t68, t9971);
        let (t13397, t13433, t13463, t13515, t13520, t13550) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1384(t13396, t226, t4265, t814, t225, t4149, t4351, t892, t1543, t2841, t4389, t699);
    (t13360, t13362, t13365, t13368, t13380, t13384, t13397, t13433, t13463, t13515, t13520, t13550)
}
