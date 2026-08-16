//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1521/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1521(t10295: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13557: f64, t13561: f64, t13642: f64, t13647: f64, t13921: f64, t13922: f64, t13923: f64) -> f64 {
    let t13931 = t10295 + 10.0_f64 / 27.0_f64 * t10296 - t10298 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t10300 - t10302 / 9.0_f64 + 5.0_f64 / 27.0_f64 * t13642 - t13921 + t13922 - t13923 + 2.0_f64 / 27.0_f64 * t13539 - t13557 / 3.0_f64 + t13530 / 9.0_f64 + t13534 / 18.0_f64 + t13561 - 2.0_f64 / 3.0_f64 * t13544 - t13548 / 3.0_f64 + t13647 / 6.0_f64;
    t13931
}
