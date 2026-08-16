//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta122 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk727;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk728;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk729;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk730;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk731;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta122(t865: f64, t2718: f64, t252: f64, t2627: f64, t2633: f64, t814: f64, t852: f64, t829: f64, t2679: f64, t860: f64, t2684: f64, t235: f64, t2710: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t808: f64, t812: f64, t861: f64, t863: f64, t858: f64, t259: f64, t2592: f64, t2594: f64, t2597: f64, t2711: f64, t2713: f64, t855: f64, t866: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2719 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk727(t865);
        let t2720 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk728(t2718, t2719);
        let (t2729, t2732, t2733, t2736, t2738, t2740, t2742) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk729(t252, t2627, t2633, t814, t852, t829, t2679, t860, t2684, t235, t2710, t226, t255, t2613, t2617, t808, t812, t861, t863);
        let t2743 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk730(t2742, t858);
        let t2745 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk731(t259, t2592, t2594, t2597, t2711, t2713, t2720, t2743, t855, t866);
        let t2749 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk732(t868);
    (t2719, t2720, t2729, t2732, t2733, t2736, t2738, t2740, t2742, t2743, t2745, t2749)
}
