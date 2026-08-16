//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1230/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1230(t1178: f64, t13783: f64, t13917: f64, t8787: f64, t51543: f64, t50998: f64, t9516: f64, t2079: f64, t898: f64, t3258: f64, t816: f64, t820: f64, t938: f64) -> (f64, f64, f64, f64) {
    let t53152 = t13917 * t1178 * t8787 * t13783;
    let t53156 = t1178 * t51543;
    let t53158 = t50998 * t53156 * t9516;
    let t53161 = t1178 * t898 * t2079;
    let t53166 = t13917 * t53161 * t3258 * t816 * t938 * t820;
    (t53152, t53156, t53158, t53166)
}
