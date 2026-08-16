//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2626/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2626<F: Float>(t48302: F, t13665: F, t9863: F, t9866: F, t47093: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F, t47096: F, t47098: F, t48291: F, t48293: F, t48295: F, t48296: F, t48298: F, t48300: F) -> (F, F, F, F, F) {
    let t48303 = F::cast_from(0.32530743900905219526e-1_f64) * t48302;
    let t48304 = t13665 * t9863;
    let t48305 = F::cast_from(0.16265371950452609763e-1_f64) * t48304;
    let t48306 = t13665 * t9866;
    let t48307 = F::cast_from(0.48159733137676571078e0_f64) * t48306;
    let t48308 = F::cast_from(0.31168546390226634765e3_f64) * t47093;
    let t48309 = -t48291 + t48293 - t48295 - t48296 - t48298 - t47084 - t48300 + t48303 + t48305 + t48307 - t39989 - t47086 + t47088 + t47092 + t48308 - t47096 - t47098;
    (t48303, t48305, t48307, t48308, t48309)
}
