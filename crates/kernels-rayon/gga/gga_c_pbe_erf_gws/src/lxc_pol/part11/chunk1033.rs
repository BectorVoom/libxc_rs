//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1033/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1033(t13045: f64, t168: f64, t703: f64, t13008: f64, t256: f64, t719: f64, t12323: f64, t19: f64, t336: f64, t714: f64, t13039: f64, t735: f64) -> (f64, f64, f64, f64) {
    let t42935 = t168 * t703 * t13045;
    let t42943 = t13008 * t719 * t256;
    let t42948 = t12323 * t19 * t336 * t714;
    let t42953 = t13039 * t735;
    (t42935, t42943, t42948, t42953)
}
