//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1109/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1109(t14136: f64, t14138: f64, t1173: f64, t2222: f64, t4116: f64, t945: f64, t2182: f64, t4066: f64, t810: f64, t2074: f64, t1206: f64, t353: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14139 = t14136 * t14138;
    let t14141 = t1173 * t2222;
    let t14161 = t4116 * t945;
    let t14166 = t4066 * t2182;
    let t14169 = t14161 * t810;
    let t14175 = t4066 * t2074;
    let t14180 = t1206 * t810;
    let t14181 = t353 * t14180;
    (t14139, t14141, t14161, t14166, t14169, t14175, t14180, t14181)
}
