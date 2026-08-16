//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1147/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1147(t1161: f64, t4052: f64, t2409: f64, t3067: f64, t1192: f64, t2494: f64, t2376: f64, t13780: f64, t3212: f64, t3990: f64, t13859: f64, t1176: f64, t367: f64, t6365: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14622 = t4052 * t1161;
    let t14624 = t2409 * t3067 * t14622;
    let t14627 = t1192 * t2494;
    let t14629 = t2409 * t2376 * t14627;
    let t14633 = t3990 * t13780 * t3212;
    let t14634 = t13859 * t14633;
    let t14637 = t1176 * t367 * t6365;
    (t14622, t14624, t14627, t14629, t14633, t14634, t14637)
}
