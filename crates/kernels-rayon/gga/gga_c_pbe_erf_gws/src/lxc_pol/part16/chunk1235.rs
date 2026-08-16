//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1235/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1235(t3952: f64, t8751: f64, t14423: f64, t14682: f64, t2158: f64, t3989: f64, t14617: f64, t50943: f64, t3990: f64, t3991: f64, t9080: f64, t345: f64, t6126: f64) -> (f64, f64, f64, f64, f64) {
    let t53266 = t3952 * t8751;
    let t53270 = t3989 * t14682 * t14423 * t2158;
    let t53272 = t50943 * t14617;
    let t53276 = t3989 * t3990 * t3991 * t9080;
    let t53283 = t345 * t6126;
    (t53266, t53270, t53272, t53276, t53283)
}
