//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk342;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk343;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk344;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk345;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta48(t300: f64, t311: f64, t890: f64, t916: f64, t919: f64, t924: f64, t933: f64, t939: f64, t943: f64, t952: f64, t315: f64, t942: f64, t950: f64, t951: f64, t338: f64, t615: f64, t134: f64, t340: f64, t344: f64, t221: f64, t339: f64, t209: f64, t39: f64, t119: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t956, t958, t959) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk342(t300, t311, t890, t916, t919, t924, t933, t939, t943, t952, t315);
        let (t961, t963, t964) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk343(t942, t950, t951, t959, t338, t615);
        let (t967, t969, t971, t972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk344(t134, t340, t344, t221, t339, t209, t338);
        let t973 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk345(t39, t972);
        let t974 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk346(t119, t60);
    (t956, t958, t959, t961, t963, t964, t967, t969, t971, t972, t973, t974)
}
