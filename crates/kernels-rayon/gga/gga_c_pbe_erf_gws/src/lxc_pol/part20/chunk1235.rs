//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1235/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1235(t53178: f64, t14452: f64, t9270: f64, t14759: f64, t4414: f64, t14633: f64, t51666: f64, t13888: f64, t3306: f64, t353: f64, t859: f64, t14404: f64, t19906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53179 = 7.0_f64 / 576.0_f64 * t53178;
    let t53187 = 7.0_f64 / 72.0_f64 * t9270 * t14452;
    let t53189 = 7.0_f64 / 72.0_f64 * t4414 * t14759;
    let t53198 = t51666 * t14633;
    let t53199 = 7.0_f64 / 576.0_f64 * t53198;
    let t53220 = t859 * t353 * t13888 * t3306;
    let t53224 = 7.0_f64 / 72.0_f64 * t19906 * t14404;
    (t53179, t53187, t53189, t53199, t53220, t53224)
}
