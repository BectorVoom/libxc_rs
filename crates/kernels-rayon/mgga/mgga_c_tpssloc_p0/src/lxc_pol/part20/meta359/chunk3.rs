//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1682/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1682(t12434: f64, t553: f64, t12169: f64, t12172: f64, t12179: f64, t12181: f64, t12238: f64, t12241: f64, t12244: f64, t12252: f64, t12256: f64, t12260: f64, t12267: f64, t12273: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t3773: f64, t3777: f64, t3898: f64, t3902: f64, t3905: f64, t3907: f64, t3909: f64, t5334: f64, t5344: f64, t544: f64, t564: f64) -> (f64, f64) {
    let t12435 = t553 * t12434;
    let t12437 = -t12169 * t1336 + 6.0_f64 * t12172 * t1336 - t12179 * t1336 - 3.0_f64 * t12181 * t1336 + t12238 * t564 + 6.0_f64 * t12241 * t5334 - 3.0_f64 * t12244 * t1336 - 6.0_f64 * t12252 * t1336 + 6.0_f64 * t12256 * t1336 - 3.0_f64 * t12260 * t1336 - 3.0_f64 * t12267 * t1381 - 3.0_f64 * t12273 * t5344 + t12435 * t544 + 3.0_f64 * t1332 * t3909 + 3.0_f64 * t1383 * t3773 + 6.0_f64 * t3777 * t3898 - 6.0_f64 * t3777 * t3902 - 3.0_f64 * t3777 * t3905 - 3.0_f64 * t3777 * t3907;
    (t12435, t12437)
}
