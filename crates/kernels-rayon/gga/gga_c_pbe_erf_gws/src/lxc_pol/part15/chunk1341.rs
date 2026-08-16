//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1341/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1341(t2370: f64, t36199: f64, t830: f64, t9296: f64, t51555: f64, t53236: f64, t8891: f64, t14617: f64, t50884: f64, t22172: f64, t2409: f64, t3965: f64) -> (f64, f64, f64, f64, f64) {
    let t54598 = t36199 * t2370;
    let t54599 = t830 * t9296;
    let t54605 = t51555 * t53236 * t8891;
    let t54607 = t50884 * t14617;
    let t54613 = t3965 * t2409 * t22172;
    (t54598, t54599, t54605, t54607, t54613)
}
