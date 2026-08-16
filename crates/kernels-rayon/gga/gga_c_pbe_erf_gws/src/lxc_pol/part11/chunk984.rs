//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 984/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk984(t10052: f64, t1243: f64, t10074: f64, t10077: f64, t3656: f64, t542: f64, t496: f64, t1251: f64, t1508: f64, t3652: f64, t3660: f64, t1552: f64, t3665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33967 = t10052 * t1243;
    let t33973 = t10074 * t1243;
    let t33975 = t10077 * t1243;
    let t34038 = t542 * t3656;
    let t34039 = t496 * t34038;
    let t34045 = t1508 * t3652 * t1251;
    let t34080 = t542 * t3660;
    let t34081 = t496 * t34080;
    let t34084 = t1552 * t3665 * t1251;
    (t33967, t33973, t33975, t34038, t34039, t34045, t34080, t34081, t34084)
}
