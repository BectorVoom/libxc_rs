//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 865/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk865<F: Float>(t2218: F, t2221: F, t2225: F, t2232: F, t1406: F, t604: F) -> (F, F) {
    let t3951 = -t2218 - F::cast_from(0.78e0_f64) * t2221 - F::cast_from(0.578e2_f64) * t2225 + t2232;
    let t3953 = t1406 * t604;
    (t3951, t3953)
}
