//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1105/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1105(t1518: f64, t2055: f64, t1936: f64, t572: f64, t28986: f64, t7553: f64, t7741: f64, t1918: f64, t2040: f64, t2115: f64, t34011: f64, t34014: f64, t34341: f64, t34346: f64, t34348: f64, t34350: f64, t34358: f64, t573: f64, t7944: f64, t8124: f64, t8127: f64, t8616: f64, t8725: f64) -> (f64, f64, f64, f64, f64) {
    let t34359 = t1518 * t2055;
    let t34360 = t34359 * t1936;
    let t34362 = 6.0_f64 * t572 * t34360;
    let t34363 = t28986 * t1936;
    let t34365 = 6.0_f64 * t572 * t34363;
    let t34366 = t7553 * t7741;
    let t34368 = 6.0_f64 * t572 * t34366;
    let t34369 = 3.0_f64 * t1918 * t8725 + 6.0_f64 * t2040 * t8124 + 3.0_f64 * t2040 * t8127 + 3.0_f64 * t2115 * t7944 + t34341 * t573 + t34011 + t34014 + t34346 + t34348 + t34350 + t34358 + t34362 + t34365 + t34368 + t8616;
    (t34359, t34360, t34363, t34366, t34369)
}
