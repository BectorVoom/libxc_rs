//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2720/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2720<F: Float>(t14383: F, t2398: F, t40108: F, t14616: F, t2619: F, t162: F, t40207: F, t4403: F, t40119: F, t40121: F, t14386: F, t2615: F) -> (F, F, F, F, F, F, F) {
    let t50045 = F::cast_from(12.0_f64) * t2398 * t14383;
    let t50046 = F::cast_from(0.5848223622634646207e0_f64) * t40108;
    let t50047 = t14616 * t2619;
    let t50048 = F::cast_from(0.73245789224026180216e-3_f64) * t50047;
    let t50051 = F::cast_from(36.0_f64) * t40207 * t162 * t4403;
    let t50055 = F::cast_from(4.0_f64) * t40119;
    let t50056 = F::cast_from(0.31168546390226634765e3_f64) * t40121;
    let t50058 = t14386 * t2615;
    (t50045, t50046, t50048, t50051, t50055, t50056, t50058)
}
