//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1387;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta256(t10046: f64, t218: f64, t225: f64, t2592: f64, t2627: f64, t852: f64, t2633: f64, t235: f64, t860: f64, t9958: f64, t2679: f64, t2732: f64, t2710: f64, t814: f64, t829: f64, t252: f64, t9971: f64, t9976: f64, t2728: f64, t9981: f64, t2684: f64, t6647: f64, t9632: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10047, t10049, t10055, t10058, t10069, t10073) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1387(t10046, t218, t225, t2592, t2627, t852, t2633, t235, t860, t9958, t2679, t2732);
        let (t10076, t10077, t10081, t10084, t10091, t10094) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1388(t2710, t814, t829, t252, t9971, t9976, t2728, t9981, t2684, t2732, t6647, t9632);
    (t10047, t10049, t10055, t10058, t10069, t10073, t10076, t10077, t10081, t10084, t10091, t10094)
}
