//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 963/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk963(t553: f64, t8309: f64, t1371: f64, t8465: f64, t3013: f64, t547: f64, t164: f64, t8279: f64, t26143: f64, t1052: f64, t163: f64, t169: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26404 = t8309 * t553;
    let t26411 = t8465 * t1371 * t553;
    let t26415 = t3013 * t547;
    let t26417 = t8279 * t164;
    let t26419 = t26143 * t164;
    let t26432 = t169 * t366 * t1052 * t163;
    (t26404, t26411, t26415, t26417, t26419, t26432)
}
