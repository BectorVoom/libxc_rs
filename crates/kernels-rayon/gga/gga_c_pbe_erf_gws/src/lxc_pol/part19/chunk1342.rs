//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1342/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1342(t3912: f64, t50887: f64, t14138: f64, t2409: f64, t35890: f64, t3965: f64, t12243: f64, t14121: f64, t1113: f64, t1161: f64, t13781: f64, t2271: f64, t3972: f64) -> (f64, f64, f64, f64) {
    let t57604 = t3912 * t50887;
    let t57605 = t57604 * t14138;
    let t57608 = t3965 * t2409 * t35890;
    let t57614 = t14121 * t12243;
    let t57626 = t3972 * t13781 * t1113 * t2271 * t1161;
    (t57605, t57608, t57614, t57626)
}
