//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1122/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1122(t12394: f64, t9702: f64, t1127: f64, t2840: f64, t11476: f64, t3931: f64, t11453: f64, t4279: f64, t1125: f64, t4233: f64, t3052: f64, t1501: f64, t3081: f64) -> (f64, f64, f64, f64, f64) {
    let t12395 = t9702 * t12394;
    let t12399 = t1127 * t2840;
    let t12400 = t12399 * t11476;
    let t12401 = t3931 * t12400;
    let t12404 = t11453 * t4279;
    let t12406 = 5.0_f64 / 10368.0_f64 * t1125 * t12404;
    let t12407 = t11453 * t4233;
    let t12409 = t3052 * t12407 / 1152.0_f64;
    let t12410 = t1501 * t3081;
    (t12395, t12401, t12406, t12409, t12410)
}
