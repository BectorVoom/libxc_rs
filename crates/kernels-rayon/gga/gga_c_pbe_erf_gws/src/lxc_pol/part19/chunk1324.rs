//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1324/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1324(t3837: f64, t51301: f64, t11585: f64, t4028: f64, t11693: f64, t51274: f64, t14058: f64, t3875: f64, t36666: f64, t850: f64, t14093: f64, t11849: f64, t14031: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57182 = t51301 * t3837;
    let t57184 = t4028 * t11585;
    let t57186 = t51274 * t11693;
    let t57188 = t14058 * t3875;
    let t57190 = t850 * t36666;
    let t57191 = t57190 * t14093;
    let t57195 = t14031 * t11849;
    (t57182, t57184, t57186, t57188, t57191, t57195)
}
