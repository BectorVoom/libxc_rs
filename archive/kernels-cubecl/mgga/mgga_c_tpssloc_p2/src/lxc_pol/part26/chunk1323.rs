//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1323/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1323<F: Float>(t1081: F, t11122: F, t1877: F, t1915: F, t1969: F, t22959: F, t23286: F, t23290: F, t23295: F, t23789: F, t23813: F, t25013: F, t2522: F, t25372: F, t3231: F, t6666: F, t6670: F, t6841: F, t6848: F, t81483: F, t81525: F, t82320: F, t83613: F, t83617: F, t83624: F, t83627: F, t83630: F, t83645: F, t83651: F) -> F {
    let t83654 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t23286 * t1081 - F::cast_from(9.0_f64) * t81483 * t23789 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t83613 + F::cast_from(3.0_f64) * t1877 * t23295 * t83617 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t23290 * t23813 - F::cast_from(9.0_f64) * t25013 * t83624 + F::cast_from(9.0_f64) * t25013 * t83627 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t6670 * t83630 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t81525 * t6848 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t23286 * t6841 + t1877 * t1915 * t11122 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t82320 * t1969 + F::cast_from(3.0_f64) * t25372 * t83645 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t6666 * t3231 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t22959 * t83651;
    t83654
}
