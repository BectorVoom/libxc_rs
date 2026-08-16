//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta45 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk325;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk326;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk327;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk328;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta45(t880: f64, t886: f64, t894: f64, t273: f64, t241: f64, t697: f64, t281: f64, t283: f64, t340: f64, t884: f64, t136: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t896 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk325(t880, t886);
        let (t897, t899, t901) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk326(t894, t896, t880, t273);
        let (t902, t904, t906) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk327(t896, t901, t241, t697, t281, t283);
        let (t907, t908) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk328(t906, t241, t340);
        let (t909, t910, t912) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk329(t884, t908, t136, t886, t897, t899, t902, t907);
    (t896, t897, t899, t901, t902, t904, t906, t907, t908, t909, t910, t912)
}
