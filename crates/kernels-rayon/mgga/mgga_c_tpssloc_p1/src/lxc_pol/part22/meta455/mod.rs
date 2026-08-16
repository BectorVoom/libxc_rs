//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1821;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1822;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta455(t1426: f64, t5392: f64, t584: f64, t9212: f64, t25: f64, t28: f64, zeta_threshold: f64, t31: f64, t65: f64, t5399: f64, t1410: f64, t5427: f64, t1409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20210, t20215, t20216) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1821(t1426, t5392, t584, t9212);
        let t20217 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1822(t25, t28, t20216, zeta_threshold);
        let (t20218, t20219, t20222, t20227, t20234) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1823(t20217, t31, t65, t1426, t5399, t1410, t5427, t1409, t5392);
    (t20210, t20215, t20216, t20217, t20218, t20219, t20222, t20227, t20234)
}
