//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1328/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1328(t11919: f64, t4049: f64, t13953: f64, t15314: f64, t11403: f64, t3959: f64, t11398: f64, t11757: f64, t3972: f64, t3975: f64, t11588: f64, t14617: f64, t53688: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57235 = t4049 * t11919;
    let t57260 = t13953 * t15314;
    let t57262 = t3959 * t11403;
    let t57265 = t3959 * t11398;
    let t57284 = t3972 * t3975 * t11757;
    let t57287 = t3972 * t3975 * t11588;
    let t57289 = t53688 * t14617;
    (t57235, t57260, t57262, t57265, t57284, t57287, t57289)
}
