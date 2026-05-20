//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 642/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk642<F: Float>(t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F, t730: F, t722: F, t164: F, t172: F, t2538: F, t123: F, t147: F, t2434: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2548 = -F::cast_from(0.78438333333333333333e0_f64) * t2502 + F::cast_from(0.15687666666666666667e1_f64) * t2504 + F::cast_from(0.68863333333333333333e0_f64) * t2435 + F::cast_from(0.14025833333333333333e0_f64) * t2509 + F::cast_from(0.28051666666666666667e0_f64) * t2511 + F::cast_from(0.17365833333333333333e0_f64) * t2439;
    let t2549 = t2548 * t730;
    let t2552 = t722 * t722;
    let t2553 = F::new(1.0) / t2552;
    let t2554 = t164 * t2553;
    let t2555 = t172 * t172;
    let t2556 = F::new(1.0) / t2555;
    let t2557 = t2538 * t2556;
    let t2562 = F::cast_from(0.14764627977777777777e-2_f64) * t123 * t2434 * t147;
    (t2548, t2549, t2552, t2553, t2554, t2555, t2556, t2557, t2562)
}
