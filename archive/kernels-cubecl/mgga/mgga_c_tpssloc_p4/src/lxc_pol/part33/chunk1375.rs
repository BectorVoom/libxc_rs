//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1375/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1375<F: Float>(t28864: F, t4028: F, t28002: F, t7468: F, t1874: F, t67001: F, t1799: F, t6463: F, t22574: F, t8643: F, t19451: F, t7461: F) -> (F, F, F, F, F) {
    let t106895 = F::cast_from(6.0_f64) * t4028 * t28864;
    let t106899 = F::cast_from(12.0_f64) * t28002 * t7468;
    let t106901 = F::cast_from(2.0_f64) * t67001 * t1874;
    let t106902 = t1799 * t6463;
    let t106905 = F::cast_from(9.0_f64) * t22574 * t8643 * t106902;
    let t106919 = F::cast_from(6.0_f64) * t19451 * t7461;
    (t106895, t106899, t106901, t106905, t106919)
}
