//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2106;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta591(t116: f64, t212: f64, t2570: f64, t2585: f64, t4255: f64, t2628: f64, t2691: f64, t4184: f64, t812: f64, t1512: f64, t41362: f64, t13176: f64, t2629: f64, t4166: f64, t9666: f64, t2693: f64, t4163: f64, t41008: f64, t4155: f64, t41115: f64, t4240: f64, t41340: f64, t4236: f64, t9671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46853, t46856, t46875, t46876, t46878) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2106(t116, t212, t2570, t2585, t4255, t2628, t2691, t4184, t812, t1512, t41362, t13176, t2629);
        let (t46881, t46887, t46912, t46929, t46952, t46953) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2107(t4166, t9666, t2693, t4163, t41008, t4155, t41115, t4240, t1512, t41340, t4236, t9671);
    (t46853, t46856, t46875, t46876, t46878, t46881, t46887, t46912, t46929, t46952, t46953)
}
