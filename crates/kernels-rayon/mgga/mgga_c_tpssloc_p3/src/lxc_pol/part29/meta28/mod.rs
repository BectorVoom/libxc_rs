//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk204;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk205;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk206;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk207;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk208;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk209;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk210;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk211;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta28(t205: f64, t215: f64, t535: f64, t131: f64, t534: f64, t221: f64, t225: f64, t144: f64, t523: f64, t525: f64, t533: f64, t68: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t539 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk204(t205, t215, t535);
        let (t540, t541, t544) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk205(t131, t534, t221, t225, t539);
        let t546 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk206(t144, t225, t523, t525);
        let t547 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk207(t533);
        let t548 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk208(t547, t68);
        let t550 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk209(t546, t548);
        let (t551, t552) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk210(t550);
        let t553 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk211(t552, t68);
        let t554 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk212(t236, t553);
    (t539, t540, t541, t544, t546, t547, t548, t550, t551, t552, t553, t554)
}
