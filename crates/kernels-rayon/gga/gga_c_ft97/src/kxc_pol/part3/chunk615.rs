//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 615/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk615(t245: f64, t1178: f64, t21: f64, t267: f64, t4431: f64, t5: f64, t5186: f64, t920: f64, t2639: f64, t992: f64, t1212: f64, t231: f64, t1218: f64, t1526: f64, t2320: f64, t2638: f64, t342: f64, t343: f64) -> (f64, f64, f64, f64) {
    let t246 = 10000000.0_f64 <= t245;
    let t5197 = piecewise3(t246, 0.0_f64, t5 * t5186 * t21 / 4.0_f64 + t5 * t1178 * t920 / 2.0_f64 + t5 * t267 * t4431 / 4.0_f64);
    let t5198 = t2639 * t992;
    let t5202 = t231 * t1212;
    let t5206 = t1218 - t2638 - t1526 * t2320 * t5198 / 12.0_f64 - t342 * t343 * t5202 / 4.0_f64;
    (t5197, t5198, t5202, t5206)
}
