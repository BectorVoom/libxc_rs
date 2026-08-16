//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 462/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk462(t1754: f64, t1706: f64, t187: f64, t190: f64, t401: f64, t607: f64, t176: f64, t177: f64, t191: f64) -> (f64, f64, f64, f64) {
    let t1844 = 0.47988888888888888889e-1_f64 * t1754;
    let t1851 = 0.11111111111111111111e-1_f64 * t190 * t1706 * t187;
    let t1852 = t401 * t607;
    let t1855 = 1.0_f64 / t177 / t176;
    let t1856 = t191 * t1855;
    (t1844, t1851, t1852, t1856)
}
