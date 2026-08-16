//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1306/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1306(t14101: f64, t8910: f64, t14024: f64, t3113: f64, t14498: f64, t9675: f64, t9494: f64, t4023: f64, t9185: f64, t14011: f64, t9681: f64, t14015: f64, t9527: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54133 = t14101 * t8910;
    let t54135 = t3113 * t14024;
    let t54136 = 7.0_f64 / 144.0_f64 * t54135;
    let t54137 = t14498 * t9675;
    let t54139 = t14498 * t9494;
    let t54142 = t9185 * t4023;
    let t54144 = t14011 * t9681;
    let t54146 = t14015 * t9527;
    (t54133, t54136, t54137, t54139, t54142, t54144, t54146)
}
