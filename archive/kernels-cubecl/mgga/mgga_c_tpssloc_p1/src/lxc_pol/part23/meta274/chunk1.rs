//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 958/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk958<F: Float>(t1336: F, t1814: F, t1838: F, t1840: F, t19815: F, t20595: F, t20616: F, t20622: F, t20625: F, t20630: F, t20632: F, t20635: F, t20638: F, t20643: F, t20645: F, t20648: F, t20651: F, t5234: F, t5334: F, t5344: F, t544: F, t564: F, t6378: F, t6448: F, t6451: F, t6454: F, t6456: F, t6458: F) -> F {
    let t20661 = -F::cast_from(6.0_f64) * t1336 * t20622 + F::cast_from(6.0_f64) * t1336 * t20625 - t1336 * t20630 - F::cast_from(3.0_f64) * t1336 * t20635 - t1336 * t20643 - F::cast_from(3.0_f64) * t1336 * t20645 - F::cast_from(3.0_f64) * t1336 * t20648 + F::cast_from(6.0_f64) * t1336 * t20651 + F::cast_from(3.0_f64) * t1814 * t6458 - F::cast_from(3.0_f64) * t1838 * t19815 + F::cast_from(3.0_f64) * t1840 * t6378 + t20595 * t564 + t20616 * t544 - F::cast_from(3.0_f64) * t20632 * t5344 + F::cast_from(6.0_f64) * t20638 * t5334 + F::cast_from(6.0_f64) * t5234 * t6448 - F::cast_from(6.0_f64) * t5234 * t6451 - F::cast_from(3.0_f64) * t5234 * t6454 - F::cast_from(3.0_f64) * t5234 * t6456;
    t20661
}
