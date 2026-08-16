//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 774/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk774(t72: f64, t9338: f64, t2245: f64, t2252: f64, t2255: f64, t2284: f64, t2304: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64, t9247: f64, t9248: f64, t9251: f64, t9260: f64, t9263: f64, t9268: f64, t9313: f64) -> (f64, f64) {
    let t9339 = t72 * t9338;
    let t9342 = -t9247 * t9248 / 4.0_f64 - t9251 * t80 / 4.0_f64 - t2245 * t642 / 4.0_f64 - t9260 * t80 / 12.0_f64 - t9263 * t80 / 4.0_f64 - t2252 * t642 / 4.0_f64 - t9268 * t80 / 4.0_f64 - t2255 * t642 / 2.0_f64 - t609 * t2304 / 4.0_f64 + t9313 * t80 / 24.0_f64 + t2284 * t642 / 8.0_f64 + t629 * t2304 / 8.0_f64 + t66 * t9339 / 24.0_f64;
    (t9339, t9342)
}
