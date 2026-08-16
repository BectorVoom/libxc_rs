//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1183/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1183(t23998: f64, t6486: f64, t1860: f64, t23992: f64, t6509: f64, t22527: f64, t22531: f64, t22534: f64, t23975: f64, t6492: f64, t7035: f64, t83832: f64, t84203: f64, t84205: f64, t84207: f64, t84209: f64, t84216: f64, t84220: f64, t84222: f64) -> f64 {
    let t84224 = t6486 * t23998;
    let t84229 = t1860 * t23992 * t6509;
    let t84231 = 32.0_f64 / 3.0_f64 * t84203 + 16.0_f64 / 3.0_f64 * t84205 + 32.0_f64 / 3.0_f64 * t84207 - 5.0_f64 * t84209 * t6492 - 10.0_f64 * t23975 * t22527 - 5.0_f64 * t23975 * t22531 - 70.0_f64 * t84216 * t83832 - 80.0_f64 * t84220 - 8.0_f64 / 3.0_f64 * t84222 - 16.0_f64 / 3.0_f64 * t84224 - 2.0_f64 * t22534 * t7035 + 88.0_f64 / 9.0_f64 * t84229;
    t84231
}
