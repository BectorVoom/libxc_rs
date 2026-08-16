//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2113;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2114;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2115;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2116;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta628(t25373: f64, t86713: f64, t25: f64, t40772: f64, t1530: f64, t2749: f64, t1408: f64, t2752: f64, t13487: f64, t22960: f64, t58071: f64, t2: f64, t584: f64, t868: f64, t25372: f64, t193: f64, t201: f64, t7540: f64, t200: f64, t6665: f64, t4303: f64, t606: f64, t1877: f64, t1915: f64, t9212: f64, t22959: f64, t22961: f64, t25013: f64, t25015: f64, t2522: f64, t25366: f64, t25375: f64, t25385: f64, t6666: f64, t6670: f64, t81483: f64, t86703: f64, t86707: f64, t86710: f64, t870: f64, t776: f64, t2553: f64, t10143: f64, t25374: f64, t25365: f64, t58009: f64, t4255: f64, t2249: f64, t22964: f64, t23286: f64, t23299: f64, t25028: f64, t25358: f64, t47645: f64, t7475: f64, t7476: f64, t7541: f64, t7545: f64, t81525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86714, t86717, t86718, t86722, t86727, t86730) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2113(t25373, t86713, t25, t40772, t1530, t2749, t1408, t2752, t13487, t22960, t58071, t2);
        let (t86734, t86736, t86740, t86746, t86751) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2114(t584, t86730, t868, t25372, t193, t201, t7540, t200, t6665, t4303, t606, t1877, t1915, t9212);
        let t86752 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2115(t1877, t22959, t22961, t25013, t25015, t2522, t25366, t25372, t25375, t25385, t6666, t6670, t81483, t86703, t86707, t86710, t86714, t86718, t86722, t86727, t86734, t86736, t86740, t86746, t86751);
        let (t86757, t86764, t86771, t86775) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2116(t2, t870, t584, t776, t22959, t1408, t2553, t10143, t606, t25374, t1877, t1915);
        let (t86781, t86797, t86801) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2117(t25365, t868, t25373, t58009, t4255, t22960, t1408, t1877, t1915, t2249, t22959, t22964, t23286, t23299, t25013, t25028, t2522, t25358, t25372, t47645, t6666, t7475, t7476, t7541, t7545, t81525, t86757, t86764, t86771, t86775);
    (t86717, t86734, t86736, t86740, t86751, t86752, t86757, t86775, t86781, t86797, t86801)
}
