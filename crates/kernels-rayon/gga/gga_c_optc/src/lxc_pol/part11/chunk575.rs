//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 575/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk575(t106: f64, t1299: f64, t167: f64, t2106: f64, t3454: f64, t4668: f64, t4675: f64, t4723: f64, t670: f64, t1303: f64, t1317: f64, t201: f64, t5: f64) -> (f64, f64, f64, f64) {
    let t4727 = 0.27818116767324025134e1_f64 * t106 * t4668 * t167 - 0.55636233534648050268e1_f64 * t106 * t3454 * t1299 + 0.55636233534648050268e1_f64 * t106 * t2106 * t4675 - 0.27818116767324025134e1_f64 * t106 * t670 * t4723;
    let t4733 = t1303 * t1303;
    let t4741 = t1317 * t1317;
    let t4743 = t5 * t4741 * t201;
    (t4727, t4733, t4741, t4743)
}
