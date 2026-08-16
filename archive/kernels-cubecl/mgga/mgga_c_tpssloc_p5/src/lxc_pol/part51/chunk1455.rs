//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1455/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1455<F: Float>(t111: F, t33578: F, t31537: F, t7802: F, t31540: F, t27226: F, t8526: F, t24999: F, t26114: F, t26179: F, t27188: F, t31532: F, t31726: F, t31734: F, t33133: F, t4028: F, t4073: F, t6515: F, t6539: F, t672: F, t6862: F, t7057: F, t7218: F, t7458: F, t7787: F, t7890: F, t8529: F) -> (F, F) {
    let t122617 = t33578 * t111;
    let t122623 = F::cast_from(2.0_f64) * t31537 * t7802;
    let t122625 = F::cast_from(2.0_f64) * t31540 * t7802;
    let t122627 = F::cast_from(2.0_f64) * t8526 * t27226;
    let t122643 = -F::cast_from(2.0_f64) * t122617 * t672 - F::cast_from(2.0_f64) * t24999 * t7057 - F::cast_from(2.0_f64) * t26114 * t8529 - F::cast_from(2.0_f64) * t26179 * t8529 - F::cast_from(2.0_f64) * t27188 * t6539 - F::cast_from(2.0_f64) * t31532 * t4073 - F::cast_from(2.0_f64) * t31726 * t7458 - F::cast_from(2.0_f64) * t31734 * t4028 + t33133 * t7218 - t6515 * t7890 - t6862 * t7787 - t122623 - t122625 - t122627;
    (t122617, t122643)
}
