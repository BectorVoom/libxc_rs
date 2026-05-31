//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2722/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2722<F: Float>(t198: F, t775: F, t10565: F, t1469: F, t706: F, t1531: F, t36: F, t10440: F, t14362: F, t9863: F, t9866: F, t40143: F) -> (F, F, F, F, F, F) {
    let t50080 = t198 * t775;
    let t50084 = t706 * t10565 * t1469;
    let t50085 = F::cast_from(4.0_f64) * t50084;
    let t50089 = t36 * t1531;
    let t50091 = F::cast_from(24.0_f64) * t50089 * t10440;
    let t50092 = t14362 * t9863;
    let t50093 = F::cast_from(0.16265371950452609763e-1_f64) * t50092;
    let t50094 = t14362 * t9866;
    let t50095 = F::cast_from(0.48159733137676571078e0_f64) * t50094;
    let t50096 = F::cast_from(36.0_f64) * t40143;
    (t50080, t50085, t50091, t50093, t50095, t50096)
}
