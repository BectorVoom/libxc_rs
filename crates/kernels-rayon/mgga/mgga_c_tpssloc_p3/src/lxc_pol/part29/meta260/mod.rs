//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1212;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1213;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1214;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1215;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1216;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1217;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta260(t1235: f64, t225: f64, t497: f64, t462: f64, t457: f64, t461: f64, t491: f64, t1240: f64, t1251: f64, t1190: f64, t2144: f64, t1193: f64, t2127: f64, t210: f64, t2120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7294, t7295) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1212(t1235, t225, t497);
        let (t7296, t7299) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1213(t462, t7295, t457, t461);
        let t7300 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1214(t491, t7299);
        let t7301 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1215(t1240, t225);
        let (t7302, t7303) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1216(t1251, t7301, t7300);
        let (t7306, t7309, t7310) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1217(t1190, t2144, t1193, t2127, t210, t2120);
    (t7294, t7295, t7296, t7299, t7300, t7301, t7302, t7303, t7306, t7309, t7310)
}
