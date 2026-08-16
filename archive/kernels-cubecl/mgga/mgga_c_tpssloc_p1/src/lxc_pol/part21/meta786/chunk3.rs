//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2729/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2729<F: Float>(t16046: F, t1814: F, t1824: F, t5318: F, t1351: F, t19735: F, t12240: F, t16033: F, t16047: F, t16048: F, t16049: F, t16052: F, t16055: F, t16125: F, t19654: F, t19660: F, t19740: F, t19743: F, t19763: F, t19810: F, t5230: F, t5250: F, t5334: F, t5335: F, t5343: F, t5345: F, t54963: F, t56666: F, t57147: F, t57499: F) -> (F, F) {
    let t57530 = t1814 * t16046;
    let t57545 = t5318 * t1824;
    let t57554 = t19735 * t1351;
    let t57564 = F::cast_from(2.0_f64) * t12240 * t19660 * t5334 - F::cast_from(36.0_f64) * t16047 * t16048 * t19743 + F::cast_from(24.0_f64) * t19743 * t54963 * t56666 - F::cast_from(4.0_f64) * t5230 * t5343 * t5345 + F::cast_from(8.0_f64) * t5250 * t5334 * t57499 + F::cast_from(8.0_f64) * t5250 * t5334 * t57545 + F::cast_from(4.0_f64) * t5334 * t5335 * t57147 + F::cast_from(24.0_f64) * t5334 * t5335 * t57554 - F::cast_from(2.0_f64) * t16033 * t19763 - F::cast_from(12.0_f64) * t16049 * t57530 + F::cast_from(12.0_f64) * t16052 * t19654 + F::cast_from(8.0_f64) * t16055 * t19740 - F::cast_from(2.0_f64) * t16125 * t19810;
    (t57545, t57564)
}
