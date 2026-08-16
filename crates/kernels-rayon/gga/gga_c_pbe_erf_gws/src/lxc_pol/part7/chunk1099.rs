//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1099/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1099(t4417: f64, t810: f64, t2370: f64, t830: f64, t2373: f64, t4474: f64, t2379: f64, t4424: f64, t2100: f64, t2395: f64, t829: f64, t2367: f64, t4402: f64) -> (f64, f64, f64, f64, f64) {
    let t19670 = t4417 * t810;
    let t19672 = t2370 * t830 * t19670;
    let t19677 = t4474 * t2373;
    let t19679 = t4424 * t2379;
    let t19683 = t829 * t830 * t2395 * t2100;
    let t19691 = t2367 * t4402;
    (t19672, t19677, t19679, t19683, t19691)
}
