//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1080/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1080(t160: f64, t165: f64, t515: f64, t86595: f64, t86598: f64, t86601: f64, t86604: f64, t87088: f64, t87091: f64, t87093: f64, t87095: f64, t87097: f64, t87163: f64, t87175: f64, t87187: f64, t87200: f64, t87214: f64) -> f64 {
    let t87219 = 16.0_f64 * t86595 + 12.0_f64 * t86598 + 48.0_f64 * t86601 - 72.0_f64 * t86604 + 2.0_f64 * t87088 * t160 - 12.0_f64 * t87091 - 48.0_f64 * t87093 + 48.0_f64 * t87095 + 24.0_f64 * t87097 - 2.0_f64 * t87163 - t515 * (t87175 + t87187 + t87200 + t87214) * t165;
    t87219
}
