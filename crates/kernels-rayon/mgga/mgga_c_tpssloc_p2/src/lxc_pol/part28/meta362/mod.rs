//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta362 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1350;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1351;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1352;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1353;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1354;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1355;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1356;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta362(t13555: f64, t2768: f64, t123: f64, t13528: f64, t13532: f64, t13559: f64, t882: f64, t13542: f64, t13546: f64, t10296: f64, t10298: f64, t10302: f64, t13567: f64, t13569: f64, t1540: f64, t2394: f64, t13563: f64, t13566: f64, t4348: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t13572 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1350(t13555, t2768, t123);
        let t13575 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1351(t13528, t2768, t123);
        let t13578 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1352(t13532, t2768, t123);
        let t13581 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1353(t13559, t882, t123);
        let t13584 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1354(t13542, t882, t123);
        let t13587 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1355(t13546, t882, t123);
        let (t13592, t13598) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1356(t10296, t10298, t10302, t13567, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t1540, t2394);
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1357(t13563, t13566, t4348, t690);
    (t13572, t13575, t13578, t13581, t13584, t13587, t13592, t13598, t13600, t13601, t13602)
}
