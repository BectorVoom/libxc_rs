//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1237/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1237(t6553: f64, t7488: f64, t1880: f64, t1492: f64, t1902: f64, t1496: f64, t6581: f64, t1484: f64, t236: f64) -> (f64, f64, f64, f64, f64) {
    let t7489 = t6553 * t7488;
    let t7490 = t1880 * t7489;
    let t7492 = t1492 * t1902;
    let t7494 = t6581 * t1496;
    let t7496 = t236 * t1484;
    (t7489, t7490, t7492, t7494, t7496)
}
