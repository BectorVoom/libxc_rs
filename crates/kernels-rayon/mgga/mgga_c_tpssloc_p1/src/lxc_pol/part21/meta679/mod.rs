//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2487;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta679(t2517: f64, t4098: f64, t1472: f64, t9862: f64, t13115: f64, t9932: f64, t32: f64, t4094: f64, t13034: f64, t225: f64, t10109: f64, t1527: f64, t13036: f64, t13336: f64, t68: f64, t1496: f64, t41083: f64, t4257: f64, t9601: f64, t13193: f64, t2697: f64, t13204: f64, t2563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46437, t46439, t46445, t46447, t46452, t46488) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2487(t2517, t4098, t1472, t9862, t13115, t9932, t32, t4094, t13034, t225, t10109, t1527);
        let (t46508, t46528, t46546, t46549, t46551, t46558) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2488(t13036, t225, t13336, t68, t1496, t41083, t4257, t9601, t13193, t2697, t13204, t2563);
    (t46437, t46439, t46445, t46447, t46452, t46488, t46508, t46528, t46546, t46549, t46551, t46558)
}
