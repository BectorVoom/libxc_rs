//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1236/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1236(t20154: f64, t3067: f64, t4164: f64, t810: f64, t14629: f64, t4414: f64, t14624: f64, t9270: f64, t14767: f64, t2373: f64, t1113: f64, t13781: f64, t2352: f64, t3972: f64, t824: f64) -> (f64, f64, f64, f64, f64) {
    let t53083 = t20154 * t3067 * t4164 * t810;
    let t53093 = 7.0_f64 / 72.0_f64 * t4414 * t14629;
    let t53099 = 7.0_f64 / 72.0_f64 * t9270 * t14624;
    let t53126 = t14767 * t2373;
    let t53131 = t3972 * t13781 * t1113 * t824 * t2352;
    (t53083, t53093, t53099, t53126, t53131)
}
