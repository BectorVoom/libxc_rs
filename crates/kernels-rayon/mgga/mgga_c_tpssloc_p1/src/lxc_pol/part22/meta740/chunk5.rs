//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2442/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2442(t300: f64, t69050: f64, t69180: f64, t69218: f64, t69249: f64, t69286: f64, t69326: f64, t69368: f64, t69449: f64, t14459: f64, t17947: f64, t959: f64) -> (f64, f64) {
    let t69453 = t300 * (t69050 + t69180 + t69218 + t69249 + t69286 + t69326 + t69368 + t69449);
    let t69456 = 0.31168546390226634765e3_f64 * t959 * t17947 * t14459;
    (t69453, t69456)
}
