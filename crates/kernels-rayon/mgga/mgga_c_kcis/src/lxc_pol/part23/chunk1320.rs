//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1320/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1320(t1640: f64, t18367: f64, t18378: f64, t1881: f64, t1884: f64, t2233: f64, t2272: f64, t27717: f64, t27720: f64, t27726: f64, t446: f64, t448: f64, t4504: f64, t5406: f64, t8130: f64, t92356: f64, t92360: f64, t92368: f64, t92375: f64, t97601: f64) -> f64 {
    let t99786 = -t446 * t18378 * t2272 / 16.0_f64 - t2233 * t5406 * t1640 / 8.0_f64 + t1881 * t27726 / 8.0_f64 - t2233 * t448 * t18367 / 16.0_f64 + t92356 + t1881 * t27717 / 16.0_f64 - t92360 + t92368 - t2233 * t1884 * t4504 / 16.0_f64 + t8130 * t27720 / 16.0_f64 - t92375 + t97601;
    t99786
}
