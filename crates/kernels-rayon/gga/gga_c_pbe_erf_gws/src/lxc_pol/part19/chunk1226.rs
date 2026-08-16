//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1226/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1226(t1112: f64, t361: f64, t51543: f64, t1178: f64, t2079: f64, t898: f64, t14705: f64, t51666: f64, t14633: f64, t1114: f64, t50942: f64, t13984: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53138 = t361 * t51543 * t1112;
    let t53156 = t1178 * t51543;
    let t53161 = t1178 * t898 * t2079;
    let t53178 = t51666 * t14705;
    let t53198 = t51666 * t14633;
    let t53229 = t1114 * t50942;
    let t53230 = t53229 * t13984;
    (t53138, t53156, t53161, t53178, t53198, t53229, t53230)
}
