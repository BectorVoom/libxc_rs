//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1067/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1067(t11984: f64, t2124: f64, t2119: f64, t3912: f64, t6342: f64, t3814: f64, t8827: f64, t9665: f64, t2083: f64, t3780: f64, t3259: f64, t3257: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11986 = t11984 * t2124 / 48.0_f64;
    let t11987 = t3912 * t2119;
    let t11989 = t11987 * t6342 / 48.0_f64;
    let t11990 = t8827 * t3814;
    let t11991 = t9665 * t11990;
    let t11994 = t3780 * t2083;
    let t11995 = t11994 * t3259;
    let t11996 = t3257 * t11995;
    (t11986, t11989, t11990, t11991, t11994, t11995, t11996)
}
