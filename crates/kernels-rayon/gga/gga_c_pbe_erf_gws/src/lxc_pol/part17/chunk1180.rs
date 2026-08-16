//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1180/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1180(t8546: f64, t944: f64, t3327: f64, t810: f64, t1198: f64, t21885: f64, t14145: f64, t945: f64, t804: f64, t8556: f64, t13757: f64, t2429: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43260 = t8546 * t944;
    let t47184 = t3327 * t810;
    let t50818 = t1198 * t21885;
    let t50825 = t14145 * t945;
    let t50832 = t804 * t1198;
    let t50833 = t50832 * t8556;
    let t50835 = t2429 * t13757;
    (t43260, t47184, t50818, t50825, t50832, t50833, t50835)
}
