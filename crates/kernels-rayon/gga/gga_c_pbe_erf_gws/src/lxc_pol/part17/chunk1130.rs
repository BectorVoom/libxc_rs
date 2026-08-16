//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1130/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1130(t14423: f64, t2171: f64, t13796: f64, t13859: f64, t2409: f64, t4007: f64, t8589: f64, t9721: f64, t3959: f64, t8708: f64, t1119: f64, t4386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14455 = t14423 * t2171;
    let t14456 = t13796 * t14455;
    let t14457 = t13859 * t14456;
    let t14460 = t2409 * t8589 * t4007;
    let t14463 = t2409 * t9721;
    let t14464 = t3959 * t14463;
    let t14466 = t2409 * t8708;
    let t14467 = t3959 * t14466;
    let t14469 = t4386 * t1119;
    (t14456, t14457, t14460, t14463, t14464, t14466, t14467, t14469)
}
