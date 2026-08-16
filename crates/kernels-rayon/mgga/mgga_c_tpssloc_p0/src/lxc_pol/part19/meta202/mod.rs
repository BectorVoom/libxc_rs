//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk871;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk872;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta202(t10321: f64, t908: f64, t136: f64, t10295: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t10307: f64, t10311: f64, t10314: f64, t10318: f64, t10320: f64, t340: f64, t343: f64, t974: f64, t2955: f64, t969: f64, t2967: f64, t964: f64, t63: f64, t344: f64, t221: f64, t339: f64, t2960: f64, t2974: f64, t3014: f64, t984: f64, t135: f64, t3016: f64, t973: f64, t10263: f64, t10267: f64, t10274: f64, t10280: f64, t10283: f64, t10287: f64, t10290: f64, t2996: f64, t3000: f64, t3011: f64, t3017: f64, t346: f64, t987: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10322, t10323, t10325) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk871(t10321, t908, t136, t10295, t10296, t10298, t10300, t10302, t10307, t10311, t10314, t10318, t10320);
        let (t10327, t10328, t10331, t10333, t10335, t10337, t10339) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk872(t10325, t340, t343, t974, t2955, t969, t2967, t964, t63, t344, t221, t339);
        let (t10346, t10348, t10349, t10352, t10357) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk873(t2960, t2974, t3014, t984, t340, t343, t974, t135, t3016, t973, t10263, t10267, t10274, t10280, t10283, t10287, t10290, t10328, t10331, t10333, t10339, t2996, t3000, t3011, t3017, t346, t987);
    (t10322, t10323, t10325, t10327, t10328, t10335, t10337, t10346, t10348, t10349, t10352, t10357)
}
