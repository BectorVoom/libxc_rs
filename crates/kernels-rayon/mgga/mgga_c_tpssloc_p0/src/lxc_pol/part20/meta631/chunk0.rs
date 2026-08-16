//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2296/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2296(t1519: f64, t2678: f64, t10091: f64, t13176: f64, t13381: f64, t13390: f64, t13431: f64, t13456: f64, t255: f64, t2617: f64, t2738: f64, t2740: f64, t41014: f64, t4162: f64, t4166: f64, t4281: f64, t4282: f64, t4291: f64, t4295: f64, t46861: f64, t812: f64, t9958: f64, t9981: f64) -> (f64, f64) {
    let t47528 = t1519 * t2678;
    let t47558 = 2.0_f64 * t41014 * t4281 * t4282 + 14.0_f64 * t4281 * t4282 * t9981 - t4282 * t4291 * t9958 - t4295 * t812 * t9958 - 3.0_f64 * t10091 * t4166 - 3.0_f64 * t13176 * t2738 - 6.0_f64 * t13381 * t13390 - 6.0_f64 * t13390 * t13456 - 3.0_f64 * t13431 * t2617 + t255 * t46861 + 3.0_f64 * t2740 * t4162;
    (t47528, t47558)
}
