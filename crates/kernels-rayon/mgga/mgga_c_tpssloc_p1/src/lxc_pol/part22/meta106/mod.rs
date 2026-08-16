//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk720;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk721;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk722;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk723;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta106(t323: f64, t300: f64, t938: f64, t964: f64, t969: f64, t615: f64, t972: f64, t340: f64, t697: f64, t344: f64, t221: f64, t339: f64, t135: f64, t976: f64, t979: f64, t973: f64, t986: f64, t271: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2931, t2932) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk720(t323);
        let t2940 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk721(t300, t938);
        let (t2958, t2960) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk722(t964, t969, t615, t972);
        let (t2965, t2967, t2969, t2970) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk723(t340, t697, t344, t221, t339, t135, t976);
        let (t2972, t2975, t2978) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk724(t2970, t979, t973, t135, t986, t271, t883);
    (t2931, t2932, t2940, t2958, t2960, t2965, t2967, t2969, t2970, t2972, t2975, t2978)
}
