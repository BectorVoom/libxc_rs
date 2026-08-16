//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1111/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1111(t1192: f64, t6781: f64, t829: f64, t830: f64, t331: f64, t816: f64, t1195: f64, t2242: f64, t326: f64, t837: f64, t867: f64) -> (f64, f64, f64, f64, f64) {
    let t13937 = t6781 * t1192;
    let t13939 = t829 * t830 * t13937;
    let t13942 = t816 * t331;
    let t13948 = 35.0_f64 / 432.0_f64 * t2242 * t1195;
    let t13952 = t326 * t837;
    let t13953 = t13952 * t867;
    (t13939, t13942, t13948, t13952, t13953)
}
