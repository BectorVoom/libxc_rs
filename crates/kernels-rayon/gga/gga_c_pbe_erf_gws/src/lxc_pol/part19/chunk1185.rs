//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1185/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1185(t2409: f64, t9883: f64, t3965: f64, t14469: f64, t14657: f64, t3825: f64, t3990: f64, t3991: f64, t3989: f64, t3855: f64, t14116: f64, t3835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15345 = t2409 * t9883;
    let t15346 = t3965 * t15345;
    let t15348 = t14657 * t14469;
    let t15357 = t3990 * t3991 * t3825;
    let t15358 = t3989 * t15357;
    let t15366 = t3990 * t3991 * t3855;
    let t15367 = t3989 * t15366;
    let t15371 = t3990 * t14116 * t3835;
    (t15345, t15346, t15348, t15357, t15358, t15366, t15367, t15371)
}
