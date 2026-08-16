//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1214/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1214(t1052: f64, t1923: f64, t23310: f64, t23314: f64, t23317: f64, t23323: f64, t23327: f64, t23333: f64, t23337: f64, t23341: f64, t23346: f64, t23381: f64, t23574: f64, t23732: f64, t3026: f64, t3169: f64, t6687: f64, t6707: f64, t6776: f64) -> f64 {
    let t23734 = -0.16449340668482264365e-1_f64 * t6687 * t23310 - 0.82246703342411321825e-2_f64 * t6687 * t23314 - 0.82246703342411321825e-2_f64 * t6687 * t23317 + 4.0_f64 * t3026 * t6776 + 0.80418998823691070228e-1_f64 * t23323 * t1923 - 0.54831135561607547884e-2_f64 * t23327 * t23333 - 0.54831135561607547884e-2_f64 * t23327 * t23337 - 6.0_f64 * t1052 * t23341 + 4.0_f64 * t3169 * t6776 + 0.43864908449286038306e-1_f64 * t23346 * t6707 + t23381 + t23574 + t23732;
    t23734
}
