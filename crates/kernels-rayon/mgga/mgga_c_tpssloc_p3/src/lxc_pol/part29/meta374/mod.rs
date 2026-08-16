//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta374 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1493;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1494;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1495;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1496;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1497;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1498;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1499;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta374(t13555: f64, t2768: f64, t123: f64, t13528: f64, t13532: f64, t13559: f64, t882: f64, t13542: f64, t13546: f64, t10296: f64, t10298: f64, t10302: f64, t13567: f64, t13569: f64, t1540: f64, t2394: f64, t13563: f64, t13566: f64, t4348: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t13572 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1493(t13555, t2768, t123);
        let t13575 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1494(t13528, t2768, t123);
        let t13578 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1495(t13532, t2768, t123);
        let t13581 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1496(t13559, t882, t123);
        let t13584 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1497(t13542, t882, t123);
        let t13587 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1498(t13546, t882, t123);
        let (t13592, t13598) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1499(t10296, t10298, t10302, t13567, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t1540, t2394);
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1500(t13563, t13566, t4348, t690);
    (t13572, t13575, t13578, t13581, t13584, t13587, t13592, t13598, t13600, t13601, t13602)
}
