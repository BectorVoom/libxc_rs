//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1071/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1071(t46974: f64, t829: f64, t830: f64, t13615: f64, t840: f64, t12198: f64, t3052: f64, t3772: f64, t898: f64, t13173: f64, t2358: f64, t3733: f64, t39470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46976 = t829 * t830 * t46974;
    let t46996 = t840 * t13615;
    let t47008 = t12198 * t3052;
    let t47050 = t898 * t3772;
    let t47071 = t13173 * t2358;
    let t47082 = t39470 * t3733;
    (t46976, t46996, t47008, t47050, t47071, t47082)
}
