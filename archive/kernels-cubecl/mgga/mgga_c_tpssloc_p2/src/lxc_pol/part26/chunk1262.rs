//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1262/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1262<F: Float>(t1877: F, t1915: F, t2249: F, t22951: F, t22959: F, t22961: F, t22964: F, t22968: F, t23286: F, t25013: F, t2522: F, t25372: F, t4314: F, t6542: F, t6666: F, t6670: F, t81470: F, t81476: F, t81483: F, t81486: F, t81489: F, t81492: F, t81501: F, t81505: F, t81509: F, t81513: F) -> F {
    let t81520 = F::cast_from(9.0_f64) * t25013 * t81470 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t23286 * t6542 + F::cast_from(9.0_f64) * t22959 * t81476 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t6666 * t2249 - F::cast_from(9.0_f64) * t81483 * t22961 - F::cast_from(9.0_f64) * t25013 * t81486 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t22959 * t81489 + F::cast_from(3.0_f64) * t25372 * t81492 + F::cast_from(9.0_f64) * t2522 * t6666 * t22964 + F::cast_from(9.0_f64) * t4314 * t6666 * t22951 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t81501 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t81505 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t81509 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t6670 * t81513 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t6666 * t22968;
    t81520
}
