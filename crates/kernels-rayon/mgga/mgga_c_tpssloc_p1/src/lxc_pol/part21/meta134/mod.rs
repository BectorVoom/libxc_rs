//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta134 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk897;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk898;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk899;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk900;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk901;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta134(t1088: f64, t3248: f64, t123: f64, t1089: f64, t2250: f64, t3237: f64, t3238: f64, t3245: f64, t423: f64, t1094: f64, t1098: f64, t1119: f64, t1097: f64, t419: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3249, t3250) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk897(t1088, t3248, t123);
        let t3252 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk898(t1089, t2250);
        let (t3253, t3254) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk899(t1088, t3252, t123);
        let (t3256, t3258, t3259) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk900(t3237, t3238, t3245, t3250, t3254, t423, t1094, t1098);
        let (t3261, t3262, t3263) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk901(t1119, t3259, t1097, t419);
        let t3264 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk902(t3263, t409);
    (t3249, t3250, t3252, t3253, t3254, t3256, t3258, t3259, t3261, t3262, t3263, t3264)
}
