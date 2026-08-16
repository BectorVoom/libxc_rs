//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1068/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1068(t1557: f64, t19238: f64, t128: f64, t1508: f64, t19: f64, t19253: f64, t156: f64, t5798: f64, t496: f64, t1504: f64, t10: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19259 = t1557 * t19238;
    let t19263 = t1508 * t128 * t19 * t19253;
    let t19264 = 0.38973666666666666666e1_f64 * t19263;
    let t19265 = t156 * t5798;
    let t19266 = t496 * t19265;
    let t19268 = t1504 * t1504;
    let t19270 = t10 * t5825 * t19268;
    (t19259, t19264, t19265, t19266, t19268, t19270)
}
