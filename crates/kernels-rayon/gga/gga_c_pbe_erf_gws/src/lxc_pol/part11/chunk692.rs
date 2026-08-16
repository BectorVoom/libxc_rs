//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 692/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk692(t19: f64, t931: f64, t329: f64, t332: f64, t838: f64, t857: f64, t1146: f64, t2242: f64, t353: f64, t858: f64, t1120: f64, t4442: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9239 = t931 * t19;
    let t9241 = t329 * t332 * t9239;
    let t9246 = t838 * t857;
    let t9270 = t329 * t9246;
    let t9275 = t2242 * t1146;
    let t9283 = t858 * t353;
    let t9290 = t4442 * t1120;
    (t9239, t9241, t9246, t9270, t9275, t9283, t9290)
}
