//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 888/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk888(t3270: f64, t3271: f64, t3236: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64) -> (f64, f64, f64) {
    let t3272 = t3270 * t3271;
    let t3274 = 4.0_f64 / 9.0_f64 * t3236;
    let t3279 = t3274 - 2.0_f64 / 9.0_f64 * t3238 - 2.0_f64 / 9.0_f64 * t3245 + 2.0_f64 / 3.0_f64 * t3250 + t3254 / 3.0_f64;
    (t3272, t3274, t3279)
}
