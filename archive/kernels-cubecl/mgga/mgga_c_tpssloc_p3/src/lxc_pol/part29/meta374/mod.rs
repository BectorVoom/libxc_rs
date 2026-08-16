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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1493;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1494;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1495;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1496;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1497;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1498;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1499;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta374<F: Float>(t13555: F, t2768: F, t123: F, t13528: F, t13532: F, t13559: F, t882: F, t13542: F, t13546: F, t10296: F, t10298: F, t10302: F, t13567: F, t13569: F, t1540: F, t2394: F, t13563: F, t13566: F, t4348: F, t690: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t13572 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1493::<F>(t13555, t2768, t123);
        let t13575 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1494::<F>(t13528, t2768, t123);
        let t13578 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1495::<F>(t13532, t2768, t123);
        let t13581 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1496::<F>(t13559, t882, t123);
        let t13584 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1497::<F>(t13542, t882, t123);
        let t13587 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1498::<F>(t13546, t882, t123);
        let (t13592, t13598) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1499::<F>(t10296, t10298, t10302, t13567, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t1540, t2394);
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1500::<F>(t13563, t13566, t4348, t690);
    (t13572, t13575, t13578, t13581, t13584, t13587, t13592, t13598, t13600, t13601, t13602)
}
