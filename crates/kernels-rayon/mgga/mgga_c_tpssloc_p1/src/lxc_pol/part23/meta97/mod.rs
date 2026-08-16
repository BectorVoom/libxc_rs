//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta97 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk542;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk543;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk544;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk545;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk546;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk547;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta97(t3400: f64, t440: f64, t448: f64, t457: f64, t697: f64, t461: f64, t221: f64, t456: f64, t1176: f64, t135: f64, t1089: f64, t405: f64, t974: f64, t3242: f64, t337: f64, t51: f64, t1887: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3401 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk542(t3400, t440);
        let (t3402, t3403) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk543(t448);
        let (t3426, t3428, t3430, t3431) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk544(t457, t697, t461, t221, t456, t1176, t135);
        let t3439 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk545(t1089, t405);
        let t3440 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk546(t3439, t974);
        let (t3441, t3447) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk547(t3242, t461, t337, t51, t1887);
        let t3448 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk548(t1176, t60);
    (t3401, t3402, t3403, t3426, t3428, t3430, t3431, t3439, t3440, t3441, t3447, t3448)
}
