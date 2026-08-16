//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1137/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1137(t14007: f64, t3291: f64, t14015: f64, t3253: f64, t1125: f64, t14064: f64, t14063: f64, t3179: f64, t854: f64, t850: f64, t8860: f64, t14093: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14529 = t14007 * t3291;
    let t14531 = t14015 * t3253;
    let t14533 = t1125 * t14064;
    let t14535 = t14063 * t3179;
    let t14536 = t854 * t14535;
    let t14538 = t850 * t8860;
    let t14539 = t14538 * t14093;
    (t14529, t14531, t14533, t14535, t14536, t14538, t14539)
}
