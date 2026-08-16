//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 819/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk819(t6363: f64, t6366: f64, t6374: f64, t6377: f64, t6379: f64, t6381: f64, t2748: f64, t471: f64, t64: f64, t90: f64, t984: f64) -> (f64, f64) {
    let t7851 = 189.0_f64 / 256.0_f64 * t6363 - 483.0_f64 / 8192.0_f64 * t6366 + 147.0_f64 / 524288.0_f64 * t6374 - 49.0_f64 / 524288.0_f64 * t6377 + 161.0_f64 / 8192.0_f64 * t6379 - 63.0_f64 / 256.0_f64 * t6381;
    let t7861 = t7851 * t471 - 8.0_f64 / 3.0_f64 * t2748 * t64 + 4.0_f64 / 3.0_f64 * t984 * t90 + 63.0_f64 / 256.0_f64 * t6363 - 49.0_f64 / 8192.0_f64 * t6366 + 49.0_f64 / 24576.0_f64 * t6379 - 21.0_f64 / 256.0_f64 * t6381;
    (t7851, t7861)
}
