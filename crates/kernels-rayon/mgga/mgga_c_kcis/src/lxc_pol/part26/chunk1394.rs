//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1394/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1394(t101750: f64, t101757: f64, t103934: f64, t103935: f64, t1299: f64, t1885: f64, t2132: f64, t2233: f64, t28325: f64, t28876: f64, t28880: f64, t446: f64, t449: f64, t5406: f64, t7570: f64, t8130: f64, t92356: f64, t92360: f64, t92368: f64, t92375: f64, t99834: f64) -> f64 {
    let t103953 = -t446 * t449 * (t103934 + t103935) / 16.0_f64 - t446 * t1885 * t28876 / 8.0_f64 + t99834 + t8130 * t28880 / 8.0_f64 + t92356 - t92360 - t2233 * t5406 * t2132 / 8.0_f64 + t92368 - t2233 * t1299 * t7570 / 16.0_f64 + t101750 - t92375 + t101757 + t8130 * t28325 / 8.0_f64;
    t103953
}
