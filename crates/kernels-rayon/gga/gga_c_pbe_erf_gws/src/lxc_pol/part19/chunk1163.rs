//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1163/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1163(t14689: f64, t14708: f64, t4227: f64, t810: f64, t2376: f64, t2409: f64, t13955: f64, t13965: f64, t14674: f64, t14678: f64, t14680: f64, t14685: f64, t14693: f64, t14699: f64, t14706: f64, t14714: f64, t2408: f64) -> (f64, f64, f64) {
    let t14974 = 7.0_f64 / 144.0_f64 * t14689;
    let t14978 = 7.0_f64 / 144.0_f64 * t14708;
    let t14979 = t4227 * t810;
    let t14981 = t2409 * t2376 * t14979;
    let t14985 = t14674 / 48.0_f64 + t14678 / 48.0_f64 + t14680 / 48.0_f64 + t14685 / 768.0_f64 - t14974 - t14693 / 1536.0_f64 + t13955 + t14699 / 384.0_f64 + t13965 + t14706 / 384.0_f64 - t14978 + t2408 * t14981 / 48.0_f64 - t14714 / 24.0_f64;
    (t14979, t14981, t14985)
}
