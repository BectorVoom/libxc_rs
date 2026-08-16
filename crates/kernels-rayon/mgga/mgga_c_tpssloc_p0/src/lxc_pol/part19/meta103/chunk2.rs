//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 571/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk571(t2970: f64, t979: f64, t973: f64, t135: f64, t986: f64, t271: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t2971 = t2970 * t979;
    let t2972 = t973 * t2971;
    let t2974 = t135 * t986;
    let t2975 = t973 * t2974;
    let t2978 = 1.0_f64 / t271 / t883;
    (t2971, t2972, t2974, t2975, t2978)
}
