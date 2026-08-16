//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1130/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1130(t6188: f64, t6411: f64, t4395: f64, t6670: f64, t2382: f64, t6680: f64, t6573: f64, t810: f64, t1452: f64, t343: f64, t874: f64, t5: f64, t6231: f64) -> (f64, f64, f64, f64, f64) {
    let t20280 = t6188 * t6411 / 16.0_f64;
    let t20281 = t4395 * t6670;
    let t20282 = t2382 * t20281;
    let t20284 = t20282 * t6680 / 8.0_f64;
    let t20285 = t6573 * t810;
    let t20291 = t1452 * t874 * t343;
    let t20296 = t5 * t6231;
    (t20280, t20284, t20285, t20291, t20296)
}
