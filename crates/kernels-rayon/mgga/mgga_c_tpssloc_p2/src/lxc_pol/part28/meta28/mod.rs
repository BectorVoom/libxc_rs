//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk198;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk199;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk200;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk201;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk202;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk203;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk204;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta28(t131: f64, t534: f64, t221: f64, t225: f64, t539: f64, t144: f64, t523: f64, t525: f64, t533: f64, t68: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t540, t541, t544) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk198(t131, t534, t221, t225, t539);
        let t546 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk199(t144, t225, t523, t525);
        let t547 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk200(t533);
        let t548 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk201(t547, t68);
        let t550 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk202(t546, t548);
        let (t551, t552) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk203(t550);
        let t553 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk204(t552, t68);
        let t554 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk205(t236, t553);
    (t540, t541, t544, t546, t547, t548, t550, t551, t552, t553, t554)
}
