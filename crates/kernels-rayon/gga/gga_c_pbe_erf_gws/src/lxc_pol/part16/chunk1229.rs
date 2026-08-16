//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1229/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1229(t1113: f64, t13781: f64, t2352: f64, t3972: f64, t824: f64, t14733: f64, t4484: f64, t1112: f64, t361: f64, t51543: f64, t13917: f64, t9388: f64) -> (f64, f64, f64) {
    let t53131 = t3972 * t13781 * t1113 * t824 * t2352;
    let t53134 = t14733 * t4484;
    let t53138 = t361 * t51543 * t1112;
    let t53140 = t13917 * t53138 * t9388;
    (t53131, t53134, t53140)
}
