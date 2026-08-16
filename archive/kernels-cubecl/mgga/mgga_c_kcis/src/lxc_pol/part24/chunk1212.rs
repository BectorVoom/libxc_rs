//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1212/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1212<F: Float>(t1657: F, t18445: F, t2169: F, t2209: F, t233: F, t29235: F, t4534: F, t5398: F, t6294: F, t7827: F, t8121: F, t911: F, t91885: F, t91895: F, t91901: F, t92157: F, t92379: F, t97561: F) -> F {
    let t99825 = -t91885 + t97561 - t2169 * t1657 * t5398 / F::cast_from(8.0_f64) + t91895 - t91901 + t92379 + t911 * t29235 / F::cast_from(8.0_f64) - t233 * t4534 * t8121 / F::cast_from(8.0_f64) - t233 * t18445 * t2209 / F::cast_from(16.0_f64) + t92157 - t233 * t6294 * t7827 / F::cast_from(16.0_f64);
    t99825
}
