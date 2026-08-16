//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1026/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1026(t1144: f64, t2418: f64, t338: f64, t2231: f64, t19: f64, t931: f64, t329: f64, t332: f64, t2409: f64, t831: f64, t8939: f64, t838: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9232 = t338 * t1144 * t2418;
    let t9236 = t338 * t1144 * t2231;
    let t9239 = t931 * t19;
    let t9241 = t329 * t332 * t9239;
    let t9243 = t2409 * t831 * t8939;
    let t9246 = t838 * t857;
    (t9232, t9236, t9239, t9241, t9243, t9246)
}
