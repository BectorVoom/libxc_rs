//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 654/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk654(t204: f64, t205: f64, t3730: f64, t2173: f64, t3017: f64, t352: f64, t1185: f64, t3033: f64, t1184: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3732 = t204 * t205 * t3730;
    let t3734 = t2173 - 0.35616666666666666666e-1_f64 * t3017 + 0.53425e-1_f64 * t3732;
    let t3736 = 0.621814e-1_f64 * t3734 * t352;
    let t3738 = 2.0_f64 * t3033 * t1185;
    let t3739 = t1184 * t1184;
    let t3740 = t3739 * t852;
    (t3732, t3734, t3736, t3738, t3739, t3740)
}
