//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1682/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1682<F: Float>(t12434: F, t553: F, t12169: F, t12172: F, t12179: F, t12181: F, t12238: F, t12241: F, t12244: F, t12252: F, t12256: F, t12260: F, t12267: F, t12273: F, t1332: F, t1336: F, t1381: F, t1383: F, t3773: F, t3777: F, t3898: F, t3902: F, t3905: F, t3907: F, t3909: F, t5334: F, t5344: F, t544: F, t564: F) -> (F, F) {
    let t12435 = t553 * t12434;
    let t12437 = -t12169 * t1336 + F::cast_from(6.0_f64) * t12172 * t1336 - t12179 * t1336 - F::cast_from(3.0_f64) * t12181 * t1336 + t12238 * t564 + F::cast_from(6.0_f64) * t12241 * t5334 - F::cast_from(3.0_f64) * t12244 * t1336 - F::cast_from(6.0_f64) * t12252 * t1336 + F::cast_from(6.0_f64) * t12256 * t1336 - F::cast_from(3.0_f64) * t12260 * t1336 - F::cast_from(3.0_f64) * t12267 * t1381 - F::cast_from(3.0_f64) * t12273 * t5344 + t12435 * t544 + F::cast_from(3.0_f64) * t1332 * t3909 + F::cast_from(3.0_f64) * t1383 * t3773 + F::cast_from(6.0_f64) * t3777 * t3898 - F::cast_from(6.0_f64) * t3777 * t3902 - F::cast_from(3.0_f64) * t3777 * t3905 - F::cast_from(3.0_f64) * t3777 * t3907;
    (t12435, t12437)
}
