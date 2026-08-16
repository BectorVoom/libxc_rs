//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk569;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk570;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk571;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta103(t964: f64, t969: f64, t615: f64, t972: f64, t340: f64, t697: f64, t344: f64, t221: f64, t339: f64, t135: f64, t976: f64, t979: f64, t973: f64, t986: f64, t271: f64, t883: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2958, t2960) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk569(t964, t969, t615, t972);
        let (t2965, t2967, t2969, t2970) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk570(t340, t697, t344, t221, t339, t135, t976);
        let (t2971, t2972, t2974, t2975, t2978) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk571(t2970, t979, t973, t135, t986, t271, t883);
        let t2979 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk572(t2978, t974);
    (t2958, t2960, t2965, t2967, t2969, t2970, t2971, t2972, t2974, t2975, t2978, t2979)
}
