//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2205/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2205<F: Float>(t1518: F, t1936: F, t2371: F, t572: F, t670: F, t7002: F, t4158: F, t7953: F, t101469: F, t117: F, t2327: F, t7741: F) -> (F, F, F, F, F) {
    let t101590 = F::cast_from(6.0_f64) * t572 * t2371 * t1936 * t1518;
    let t101594 = F::cast_from(12.0_f64) * t572 * t670 * t7002 * t1518;
    let t101598 = F::cast_from(3.0_f64) * t4158 * t7953;
    let t101601 = F::cast_from(3.0_f64) * t572 * t117 * t101469;
    let t101606 = F::cast_from(6.0_f64) * t572 * t2327 * t7741;
    (t101590, t101594, t101598, t101601, t101606)
}
