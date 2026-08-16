//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2548;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta712(t3038: f64, t49650: f64, t1041: f64, t13611: f64, t248: f64, t3051: f64, t14137: f64, t3117: f64, t10413: f64, t10422: f64, t14125: f64, t10965: f64, t4571: f64, t1020: f64, t10508: f64, t4650: f64, t10962: f64, t4630: f64, t13961: f64, t3114: f64, t10957: f64, t13950: f64, t3048: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49771, t49799, t49801, t49808, t49810) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2548(t3038, t49650, t1041, t13611, t248, t3051, t14137, t3117, t10413, t10422, t14125, t10965, t4571);
        let (t49818, t49820, t49822, t49827, t49829) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2549(t1020, t10508, t248, t4650, t10962, t4630, t13961, t3114, t10957, t4571, t13950, t3048);
    (t49771, t49799, t49801, t49808, t49810, t49818, t49820, t49822, t49827, t49829)
}
