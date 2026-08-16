//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1299/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1299(t51555: f64, t53236: f64, t8891: f64, t14617: f64, t50884: f64, t22172: f64, t2409: f64, t3965: f64, t14692: f64, t3979: f64, t4135: f64, t51966: f64) -> (f64, f64, f64, f64, f64) {
    let t54605 = t51555 * t53236 * t8891;
    let t54607 = t50884 * t14617;
    let t54613 = t3965 * t2409 * t22172;
    let t54616 = t3979 * t14692;
    let t54621 = t51966 * t4135;
    (t54605, t54607, t54613, t54616, t54621)
}
