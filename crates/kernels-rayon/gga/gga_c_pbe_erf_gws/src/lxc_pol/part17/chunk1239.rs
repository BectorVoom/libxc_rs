//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1239/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1239(t1178: f64, t51543: f64, t50998: f64, t9516: f64, t2079: f64, t898: f64, t13917: f64, t3258: f64, t816: f64, t820: f64, t938: f64, t13780: f64, t13859: f64, t3990: f64, t8754: f64) -> (f64, f64, f64, f64) {
    let t53156 = t1178 * t51543;
    let t53158 = t50998 * t53156 * t9516;
    let t53161 = t1178 * t898 * t2079;
    let t53166 = t13917 * t53161 * t3258 * t816 * t938 * t820;
    let t53170 = t13859 * t3990 * t13780 * t8754;
    (t53156, t53158, t53166, t53170)
}
