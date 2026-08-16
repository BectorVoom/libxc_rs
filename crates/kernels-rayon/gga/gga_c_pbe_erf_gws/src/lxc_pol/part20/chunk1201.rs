//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1201/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1201(t1105: f64, t814: f64, t3199: f64, t898: f64, t376: f64, t745: f64, t1114: f64, t19905: f64, t2409: f64, t857: f64, t338: f64, t885: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26623 = t1105 * t814;
    let t26654 = t3199 * t898;
    let t26730 = t376 * t745;
    let t26958 = t1114 * t19905;
    let t27047 = t857 * t2409;
    let t27105 = t885 * t338;
    (t26623, t26654, t26730, t26958, t27047, t27105)
}
