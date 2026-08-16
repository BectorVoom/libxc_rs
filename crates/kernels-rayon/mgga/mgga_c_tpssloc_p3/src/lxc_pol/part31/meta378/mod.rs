//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1333;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1334;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1335;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1336;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1337;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1338;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta378(t10216: f64, t5392: f64, t607: f64, t10564: f64, t123: f64, t10277: f64, t2768: f64, t3966: f64, t4337: f64, t5682: f64, t690: f64, t5677: f64, t882: f64, t4342: f64, t5686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17152, t17154) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1333(t10216, t5392, t607, t10564, t123);
        let (t17157, t17159) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1334(t10277, t5392, t607, t2768, t123);
        let (t17161, t17163) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1335(t3966, t4337, t2768, t123);
        let t17165 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1336(t5682, t690);
        let (t17167, t17169) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1337(t5677, t607, t882, t123);
        let (t17171, t17173) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1338(t3966, t4342, t882, t123);
        let t17175 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1339(t5686, t690);
    (t17152, t17154, t17157, t17159, t17161, t17163, t17165, t17167, t17169, t17171, t17173, t17175)
}
