//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta30 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk214;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk215;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk216;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk217;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk218;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta30(t144: f64, t193: f64, t523: f64, t525: f64, t533: f64, t571: f64, t113: f64, t510: f64, t513: f64, t111: f64, t112: f64, t11: f64, t2: f64, t10: f64, t3: f64, t9: f64, t16: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t574 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk214(t144, t193, t523, t525, t533, t571);
        let t576 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk215(t113, t510, t513, t574);
        let t577 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk216(t111, t112);
        let (t580, t581, t582, t583) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk217(t576, t577, t11, t2, t10, t3);
        let t584 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk218(t583);
        let (t586, t587, t588) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk219(t584, t9, t2, t16);
    (t574, t576, t577, t580, t581, t582, t583, t584, t586, t587, t588)
}
