//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 926/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk926(t10365: f64, t610: f64, t1885: f64, t1820: f64, t1648: f64, t3527: f64, t591: f64, t9788: f64, t590: f64, t587: f64, t3531: f64, t1802: f64, t3454: f64) -> (f64, f64, f64, f64, f64) {
    let t10366 = t10365 * t610;
    let t10367 = t1885 * t10366;
    let t10369 = 4.0_f64 / 15.0_f64 * t1820 * t10367;
    let t10371 = 4.0_f64 / 45.0_f64 * t1648 * t3527;
    let t10372 = t591 * t9788;
    let t10373 = t590 * t10372;
    let t10375 = 4.0_f64 / 45.0_f64 * t587 * t10373;
    let t10377 = 4.0_f64 / 27.0_f64 * t1648 * t3531;
    let t10378 = t1802 * t3454;
    (t10369, t10371, t10375, t10377, t10378)
}
