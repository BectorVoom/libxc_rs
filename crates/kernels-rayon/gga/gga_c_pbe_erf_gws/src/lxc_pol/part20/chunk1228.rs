//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1228/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1228(t14392: f64, t804: f64, t1198: f64, t321: f64, t43260: f64, t14380: f64, t14835: f64, t14817: f64, t2053: f64, t14387: f64, t2429: f64, t6926: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52836 = 6.0_f64 * t804 * t14392;
    let t52853 = 4.0_f64 * t321 * t1198 * t43260;
    let t52855 = 6.0_f64 * t804 * t14380;
    let t52860 = 2.0_f64 * t321 * t14835;
    let t52861 = t14817 * t2053;
    let t52884 = 6.0_f64 * t804 * t14387;
    let t52887 = 12.0_f64 * t2429 * t1198 * t6926;
    (t52836, t52853, t52855, t52860, t52861, t52884, t52887)
}
