//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1282/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1282(t1352: f64, t5348: f64, t5318: f64, t553: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t1814: f64, t1838: f64, t1840: f64, t3777: f64, t5230: f64, t5234: f64, t5334: f64, t5336: f64, t5339: f64, t5341: f64, t5344: f64, t5345: f64, t544: f64, t564: f64) -> (f64, f64, f64) {
    let t5349 = t5348 * t1352;
    let t5351 = t553 * t5318;
    let t5353 = t1332 * t1840 - t1336 * t5339 - t1336 * t5341 - t1336 * t5349 - t1381 * t5234 + t1383 * t1814 - t1838 * t3777 + t5230 * t564 + 2.0_f64 * t5334 * t5336 - t5344 * t5345 + t5351 * t544;
    (t5349, t5351, t5353)
}
