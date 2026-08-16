//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1112/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1112(t19775: f64, t824: f64, t822: f64, t6797: f64, t376: f64, t6161: f64, t353: f64, t4386: f64, t810: f64, t2366: f64, t6106: f64, t833: f64) -> (f64, f64, f64) {
    let t19905 = t824 * t19775;
    let t19906 = t822 * t19905;
    let t19907 = t19906 * t6797;
    let t19911 = t376 * t6161;
    let t19914 = t4386 * t353 * t19911 * t810;
    let t19923 = t6106 * t2366 * t833;
    (t19907, t19914, t19923)
}
