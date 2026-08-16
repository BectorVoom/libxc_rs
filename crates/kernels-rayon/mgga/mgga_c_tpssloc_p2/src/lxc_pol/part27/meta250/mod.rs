//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta250 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1211;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1212;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1213;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1214;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1215;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1216;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1217;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta250(t362: f64, t381: f64, t884: f64, t6784: f64, t1949: f64, t986: f64, t334: f64, t371: f64, t38: f64, t131: f64, t350: f64, t1009: f64, t344: f64, t1014: f64, t360: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6785 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1211(t362, t381);
        let t6786 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1212(t6785, t884);
        let t6787 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1213(t6784, t6786);
        let t6790 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1214(t1949, t986);
        let (t6793, t6794, t6795, t6796, t6797) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1215(t334, t371, t38, t131, t350);
        let t6799 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1216(t1009, t344, t1014);
        let t6800 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1217(t360, t68);
    (t6785, t6786, t6787, t6790, t6793, t6794, t6795, t6796, t6797, t6799, t6800)
}
