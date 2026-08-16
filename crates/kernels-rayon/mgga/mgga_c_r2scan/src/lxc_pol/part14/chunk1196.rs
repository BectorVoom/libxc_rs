//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1196/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1196(t11336: f64, t40594: f64, t40595: f64, t1115: f64, t39190: f64, t39192: f64, t1146: f64, t2449: f64, t2881: f64, t3560: f64, t3570: f64, t41240: f64, t41243: f64, t41247: f64, t41251: f64, t41254: f64, t41256: f64, t41258: f64, t41261: f64, t41263: f64, t41265: f64, t41270: f64, t8306: f64) -> (f64, f64, f64) {
    let t41273 = 45.0_f64 / 32.0_f64 * t40594 * t11336 * t40595;
    let t41276 = 135.0_f64 / 32.0_f64 * t39190 * t1115 * t39192;
    let t41277 = t1146 * t8306 + 2.0_f64 * t2449 * t3570 + 2.0_f64 * t2881 * t3560 + t41240 - t41243 - t41247 + t41251 + t41254 + t41256 + t41258 + t41261 - t41263 + t41265 - t41270 - t41273 + t41276;
    (t41273, t41276, t41277)
}
