//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1187/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1187(t1477: f64, t345: f64, t56: f64, t859: f64, t854: f64, t2407: f64, t810: f64, t814: f64, t858: f64, t14024: f64, t2115: f64, t2087: f64) -> (f64, f64, f64, f64, f64) {
    let t51221 = t345 * t1477 * t56 * t859;
    let t51222 = t854 * t51221;
    let t51237 = t2407 * t858 * t814 * t810;
    let t51244 = t2115 * t14024;
    let t51252 = t2087 * t14024;
    (t51221, t51222, t51237, t51244, t51252)
}
