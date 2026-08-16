//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1274/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1274(t22331: f64, t22351: f64, t833: f64, t852: f64, t1306: f64, t22162: f64, t22164: f64, t22167: f64, t22169: f64, t22171: f64, t22175: f64, t22184: f64, t22188: f64, t22313: f64, t2461: f64, t3282: f64, t6362: f64) -> (f64, f64) {
    let t22355 = 1.0_f64 * t833 * (t22331 + t22351) * t852;
    let t22356 = 6.0_f64 * t1306 * t2461 * t3282 * t6362 + t22162 + t22164 + t22167 - t22169 + t22171 + t22175 - t22184 - t22188 - t22313 + t22355;
    (t22355, t22356)
}
