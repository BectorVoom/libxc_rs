//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta246 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1182;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1183;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1184;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1185;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1186;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1187;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1188;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1189;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta246(t6689: f64, t6691: f64, t1922: f64, t986: f64, t1049: f64, t225: f64, t387: f64, t345: f64, t340: f64, t344: f64, t381: f64, t1054: f64, t1065: f64, t1945: f64, t990: f64, t131: f64, t6679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6692 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1182(t6689, t6691);
        let t6695 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1183(t1922, t986);
        let (t6698, t6699) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1184(t1049, t225, t387);
        let (t6700, t6703) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1185(t345, t6699, t340, t344);
        let t6704 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1186(t381, t6703);
        let t6705 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1187(t1054, t225);
        let (t6706, t6707) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1188(t1065, t6705, t6704);
        let (t6710, t6712) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1189(t1945, t990, t131, t6679);
    (t6692, t6695, t6698, t6699, t6700, t6703, t6704, t6705, t6706, t6707, t6710, t6712)
}
