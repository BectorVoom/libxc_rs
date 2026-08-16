//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1819;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta454(t20118: f64, t20147: f64, t3: f64, t112: f64, t6470: f64, t576: f64, t671: f64, t1458: f64, t4072: f64, t5493: f64, t12524: f64, t1401: f64, t16521: f64, t16524: f64, t19534: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t5456: f64, t577: f64, t9211: f64, t9213: f64, t9215: f64, t9217: f64, t9219: f64, t9221: f64, t9225: f64, t1437: f64, t5389: f64, t5445: f64, t1864: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20148, t20149, t20162, t20173, t20176, t20181, t20186) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1819(t20118, t20147, t3, t112, t6470, t576, t671, t1458, t4072, t5493, t12524, t1401, t16521, t16524, t19534, t3938, t3941, t5371, t5376, t5456, t577);
        let (t20193, t20201, t20204, t20207) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1820(t9211, t9213, t9215, t9217, t9219, t9221, t9225, t1437, t5389, t5445, t1864, t5398);
    (t20148, t20149, t20162, t20173, t20176, t20181, t20186, t20193, t20201, t20204, t20207)
}
