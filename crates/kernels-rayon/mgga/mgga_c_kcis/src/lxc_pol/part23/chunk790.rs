//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 790/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk790(t12140: f64, t3980: f64, t1368: f64, t25: f64, t4002: f64, t493: f64, t1377: f64, t3970: f64) -> (f64, f64, f64) {
    let t12141 = t12140 * t3980;
    let t12142 = t1368 * t12141;
    let t12144 = t25 * t4002;
    let t12145 = t493 * t12144;
    let t12147 = t3970 * t1377;
    (t12142, t12145, t12147)
}
