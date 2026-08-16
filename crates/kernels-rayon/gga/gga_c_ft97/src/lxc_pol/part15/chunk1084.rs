//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1084/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1084(t1053: f64, t167: f64, t1901: f64, t20655: f64, t20709: f64, t20723: f64, t20727: f64, t3578: f64, t40792: f64, t41002: f64, t446: f64, t4668: f64, t4714: f64, t4823: f64, t569: f64, t574: f64, t605: f64, t63586: f64, t76623: f64, t85516: f64, t85546: f64, t85554: f64, t9144: f64, t925: f64, t9327: f64, t9432: f64) -> f64 {
    let t87441 = 2.0_f64 / 3.0_f64 * t446 * t569 * t167 * t85546 - 80.0_f64 / 243.0_f64 * t446 * t41002 * t167 * t85554 + 8.0_f64 / 9.0_f64 * t76623 - 12.0_f64 * t446 * t9432 * t167 * t4668 * t4714 + 4.0_f64 * t446 * t574 * t3578 * t20723 + 4.0_f64 * t446 * t574 * t3578 * t20727 + 4.0_f64 / 3.0_f64 * t446 * t574 * t605 * t20655 * t1053 + 40.0_f64 / 27.0_f64 * t446 * t9327 * t167 * t85516 + 8.0_f64 / 3.0_f64 * t1901 * t40792 * t20709 * t925 - 4.0_f64 / 3.0_f64 * t1901 * t9144 * t20727 * t925 + 4.0_f64 / 3.0_f64 * t1901 * t63586 * t4823;
    t87441
}
