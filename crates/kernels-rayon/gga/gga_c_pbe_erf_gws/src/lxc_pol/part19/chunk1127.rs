//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1127/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1127(t4113: f64, t840: f64, t13988: f64, t4099: f64, t9270: f64, t2053: f64, t4116: f64, t1211: f64, t6854: f64, t1105: f64, t944: f64, t13919: f64, t3227: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14333 = t840 * t4113;
    let t14338 = 35.0_f64 / 216.0_f64 * t13988;
    let t14345 = t9270 * t4099;
    let t14364 = t4116 * t2053;
    let t14368 = t1211 * t6854;
    let t14383 = t1105 * t944;
    let t14415 = t13919 * t3227;
    (t14333, t14338, t14345, t14364, t14368, t14383, t14415)
}
