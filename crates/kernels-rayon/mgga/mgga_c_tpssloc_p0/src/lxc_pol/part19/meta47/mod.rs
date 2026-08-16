//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta47 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk314;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk315;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk316;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk317;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta47(t315: f64, t942: f64, t880: f64, t906: f64, t886: f64, t897: f64, t902: f64, t910: f64, t323: f64, t300: f64, t311: f64, t890: f64, t916: f64, t919: f64, t924: f64, t933: f64, t939: f64, t338: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t943, t950) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk314(t315, t942, t880, t906, t886, t897, t902, t910);
        let t951 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk315(t323);
        let t952 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk316(t950, t951);
        let (t956, t958, t959) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk317(t300, t311, t890, t916, t919, t924, t933, t939, t943, t952, t315);
        let (t961, t963, t964) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk318(t942, t950, t951, t959, t338, t615);
    (t943, t950, t951, t952, t956, t958, t959, t961, t963, t964)
}
