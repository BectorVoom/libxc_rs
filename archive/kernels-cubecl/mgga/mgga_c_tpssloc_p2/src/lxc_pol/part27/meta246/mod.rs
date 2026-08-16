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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1182;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1183;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1184;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1185;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1186;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1187;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1188;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1189;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta246<F: Float>(t6689: F, t6691: F, t1922: F, t986: F, t1049: F, t225: F, t387: F, t345: F, t340: F, t344: F, t381: F, t1054: F, t1065: F, t1945: F, t990: F, t131: F, t6679: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6692 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1182::<F>(t6689, t6691);
        let t6695 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1183::<F>(t1922, t986);
        let (t6698, t6699) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1184::<F>(t1049, t225, t387);
        let (t6700, t6703) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1185::<F>(t345, t6699, t340, t344);
        let t6704 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1186::<F>(t381, t6703);
        let t6705 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1187::<F>(t1054, t225);
        let (t6706, t6707) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1188::<F>(t1065, t6705, t6704);
        let (t6710, t6712) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1189::<F>(t1945, t990, t131, t6679);
    (t6692, t6695, t6698, t6699, t6700, t6703, t6704, t6705, t6706, t6707, t6710, t6712)
}
