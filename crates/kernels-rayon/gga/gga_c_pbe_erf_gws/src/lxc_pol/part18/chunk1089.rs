//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1089/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1089(t12169: f64, t338: f64, t353: f64, t1161: f64, t8713: f64, t4386: f64, t3739: f64, t6832: f64, t3907: f64, t845: f64, t2503: f64, t3083: f64) -> (f64, f64, f64, f64, f64) {
    let t12171 = t338 * t353 * t12169;
    let t12180 = t8713 * t1161;
    let t12181 = t353 * t12180;
    let t12182 = t4386 * t12181;
    let t12187 = t6832 * t3739;
    let t12191 = t338 * t3907 * t845;
    let t12195 = t3083 * t2503;
    (t12171, t12182, t12187, t12191, t12195)
}
