//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1213/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1213(t1195: f64, t6729: f64, t2222: f64, t3955: f64, t13953: f64, t13976: f64, t1176: f64, t2298: f64, t923: f64, t13832: f64, t51649: f64, t867: f64) -> (f64, f64, f64, f64, f64) {
    let t51957 = 455.0_f64 / 1296.0_f64 * t6729 * t1195;
    let t51958 = t3955 * t2222;
    let t51960 = t13953 * t13976;
    let t51963 = t1176 * t923 * t2298;
    let t51964 = t51963 * t13832;
    let t51966 = t51649 * t867;
    (t51957, t51958, t51960, t51964, t51966)
}
