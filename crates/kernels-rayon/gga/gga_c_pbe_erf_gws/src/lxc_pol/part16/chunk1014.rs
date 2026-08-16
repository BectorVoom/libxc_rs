//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1014/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1014(t2170: f64, t3131: f64, t6220: f64, t2168: f64, t6510: f64, t2195: f64, t3178: f64, t9037: f64, t9039: f64, t9041: f64, t9042: f64, t9084: f64, t9086: f64, t9090: f64, t9094: f64, t9096: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9098 = t2170 * t3131 * t6220;
    let t9100 = t2168 * t9098 / 48.0_f64;
    let t9101 = 7.0_f64 / 48.0_f64 * t6510;
    let t9103 = t2170 * t3178 * t2195;
    let t9105 = t2168 * t9103 / 48.0_f64;
    let t9106 = t9037 - t9039 - t9041 - t9042 - t9084 + t9086 + t9090 + t9094 - t9096 + t9100 - t9101 + t9105;
    (t9098, t9100, t9101, t9103, t9105, t9106)
}
