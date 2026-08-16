//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1084/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1084(t11273: f64, t3160: f64, t2862: f64, t3128: f64, t1042: f64, t2853: f64, t3181: f64, t999: f64, t2866: f64, t914: f64, t936: f64, t2869: f64, t2919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11277 = t11273 * t3160;
    let t11280 = t3128 * t2862;
    let t11281 = t1042 * t11280;
    let t11285 = t3181 * t999 * t2853;
    let t11286 = t1042 * t11285;
    let t11289 = t2866 * t914;
    let t11291 = 3.0_f64 * t11289 * t936;
    let t11293 = 3.0_f64 * t2869 * t2919;
    (t11277, t11280, t11281, t11285, t11286, t11289, t11291, t11293)
}
