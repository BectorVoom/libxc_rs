//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1295/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1295(t1123: f64, t3752: f64, t50998: f64, t51021: f64, t938: f64, t1134: f64, t3068: f64, t3972: f64, t53240: f64, t3902: f64, t4386: f64, t13792: f64) -> (f64, f64, f64) {
    let t56505 = t50998 * t51021 * t1123 * t3752 * t938;
    let t56511 = t3972 * t53240 * t1134 * t3068;
    let t56513 = t4386 * t3902;
    let t56514 = t13792 * t56513;
    (t56505, t56511, t56514)
}
