//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1480/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1480<F: Float>(t1256: F, t1763: F, t193: F, t336: F, t43706: F, t4700: F, t71101: F, t78344: F, t78348: F, t78355: F, t78357: F, t78359: F, t78361: F, t78364: F, t78367: F, t78370: F, t78373: F, t78646: F, t79005: F, t79533: F) -> F {
    let t79538 = -t78344 - F::cast_from(4.0_f64) * t4700 * t71101 * t1763 - F::cast_from(6.0_f64) * t193 * t336 * t78348 * t43706 + t78355 - t78357 - t78359 + t78361 - t78364 - t78367 + t78370 + t78373 + t193 * t336 * (t78646 + t79533) * t1256 + t79005;
    t79538
}
