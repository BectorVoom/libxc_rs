//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk670;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk671;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk672;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta119(t3031: f64, t3032: f64, t371: f64, t335: f64, t368: f64, t1015: f64, t1030: f64, t372: f64, t364: f64, t354: f64, t1043: f64, t121: f64, t248: f64, t884: f64, t1041: f64, t283: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3033, t3034, t3036) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk670(t3031, t3032, t371, t335);
        let (t3037, t3038, t3039) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk671(t3036, t368, t1015, t3033);
        let (t3047, t3048, t3051) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk672(t1030, t372, t364, t354, t1043, t121);
        let (t3053, t3054, t3061) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk673(t248, t3051, t884, t1041, t283, t883);
    (t3033, t3034, t3036, t3037, t3038, t3039, t3047, t3048, t3051, t3053, t3054, t3061)
}
