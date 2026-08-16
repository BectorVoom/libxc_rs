//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1082/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1082(t11269: f64, t1526: f64, t1527: f64, t15567: f64, t16633: f64, t16640: f64, t20022: f64, t20031: f64, t20039: f64, t20545: f64, t20556: f64, t20560: f64, t20568: f64, t3088: f64, t41318: f64, t41349: f64, t78678: f64, t78681: f64, t8766: f64) -> f64 {
    let t87285 = -t1526 * t3088 * t20545 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t11269 * t41318 * t20022 - t1526 * t1527 * t20560 / 4.0_f64 - t1526 * t1527 * t8766 * t20022 / 2.0_f64 + t15567 * t16640 * t20039 / 2.0_f64 + t1526 * t1527 * t20568 / 2.0_f64 - t1526 * t1527 * t20556 / 4.0_f64 - t78678 / 9.0_f64 - t78681 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t1526 * t3088 * t41349 * t20022 - t15567 * t16633 * t20031 / 3.0_f64;
    t87285
}
