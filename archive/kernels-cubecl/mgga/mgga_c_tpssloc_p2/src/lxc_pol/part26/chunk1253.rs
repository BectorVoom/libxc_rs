//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1253/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1253<F: Float>(t22638: F, t81159: F, t22892: F, t6891: F, t80645: F, t6892: F, t81186: F, t12023: F, t12026: F, t12237: F, t1323: F, t1375: F, t2006: F, t22630: F, t22670: F, t22870: F, t22913: F, t26224: F, t26225: F, t3758: F, t3882: F, t3887: F, t3911: F, t3912: F, t568: F, t6958: F, t6992: F) -> F {
    let t81350 = t81159 * t22638;
    let t81365 = t22892 * t80645 * t6891;
    let t81375 = t81186 * t6892;
    let t81377 = -F::cast_from(0.23029076935875170111e0_f64) * t81350 + F::cast_from(6.0_f64) * t1375 * t3887 * t6992 * t3911 + F::cast_from(3.0_f64) * t1323 * t22870 * t568 + t12237 * t2006 * t568 - F::cast_from(18.0_f64) * t26224 * t26225 * t12026 + F::cast_from(0.49348022005446793095e-1_f64) * t81365 - F::cast_from(3.0_f64) * t22670 * t3912 + F::cast_from(6.0_f64) * t3758 * t22913 - F::cast_from(18.0_f64) * t3882 * t22630 - F::cast_from(6.0_f64) * t6958 * t12023 - F::cast_from(0.38381794893125283518e0_f64) * t81375;
    t81377
}
