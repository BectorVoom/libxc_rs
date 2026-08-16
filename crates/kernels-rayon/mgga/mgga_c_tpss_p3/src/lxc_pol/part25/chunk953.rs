//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 953/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk953(t1098: f64, t12384: f64, t1561: f64, t3054: f64, t1127: f64, t2840: f64, t11453: f64, t4279: f64, t1125: f64, t4233: f64, t3052: f64, t1569: f64, t2719: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12385 = t1098 * t12384;
    let t12387 = t1561 * t3054;
    let t12399 = t1127 * t2840;
    let t12404 = t11453 * t4279;
    let t12406 = 5.0_f64 / 10368.0_f64 * t1125 * t12404;
    let t12407 = t11453 * t4233;
    let t12409 = t3052 * t12407 / 1152.0_f64;
    let t12429 = t1569 * t2719;
    (t12385, t12387, t12399, t12406, t12409, t12429)
}
