//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 710/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk710(t202: f64, t3477: f64, t184: f64, t3345: f64, t572: f64, t1663: f64, t3346: f64, t1022: f64, t7483: f64, t3530: f64, t5283: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10418 = t202 * t3477;
    let t10419 = t10418 * t184;
    let t10424 = t3345 * t572;
    let t10442 = t1663 * t3346;
    let t10465 = t7483 * t1022;
    let t10472 = t5283 * t3530;
    let t10473 = t587 * t10472;
    (t10418, t10419, t10424, t10442, t10465, t10472, t10473)
}
