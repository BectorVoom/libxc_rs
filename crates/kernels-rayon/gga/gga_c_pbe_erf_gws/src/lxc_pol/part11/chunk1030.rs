//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1030/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1030(t12955: f64, t395: f64, t501: f64, t12930: f64, t156: f64, t496: f64, t42658: f64, t8135: f64, t12937: f64, t12929: f64, t1563: f64, t12962: f64, t513: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42678 = t501 * t12955 * t395;
    let t42680 = t156 * t12930;
    let t42681 = t496 * t42680;
    let t42683 = t8135 * t42658;
    let t42714 = t496 * t156 * t12937;
    let t42719 = t1563 * t12929;
    let t42742 = t12962 * t513;
    (t42678, t42680, t42681, t42683, t42714, t42719, t42742)
}
