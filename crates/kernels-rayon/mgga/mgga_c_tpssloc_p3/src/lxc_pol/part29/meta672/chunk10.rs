//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2258/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2258(t1388: f64, t25988: f64, t22574: f64, t26162: f64, t26149: f64, t6876: f64, t19577: f64, t31035: f64, t25971: f64, t83886: f64, t23831: f64, t4028: f64) -> (f64, f64, f64, f64, f64) {
    let t91565 = t25988 * t1388;
    let t91568 = 12.0_f64 * t22574 * t26162 * t91565;
    let t91570 = 2.0_f64 * t6876 * t26149;
    let t91573 = 6.0_f64 * t22574 * t31035 * t19577;
    let t91578 = 6.0_f64 * t83886 * t25971;
    let t91580 = 2.0_f64 * t4028 * t23831;
    (t91568, t91570, t91573, t91578, t91580)
}
