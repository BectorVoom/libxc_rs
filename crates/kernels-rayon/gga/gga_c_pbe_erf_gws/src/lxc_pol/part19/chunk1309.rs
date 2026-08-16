//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1309/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1309(t11554: f64, t14015: f64, t11764: f64, t54119: f64, t11560: f64, t14007: f64, t11526: f64, t51421: f64, t3123: f64, t9127: f64, t11548: f64, t12015: f64, t14031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56960 = t14015 * t11554;
    let t56962 = t54119 * t11764;
    let t56964 = t14007 * t11560;
    let t56966 = t51421 * t11526;
    let t56968 = t3123 * t9127;
    let t56970 = t14007 * t11548;
    let t56972 = t14031 * t12015;
    (t56960, t56962, t56964, t56966, t56968, t56970, t56972)
}
