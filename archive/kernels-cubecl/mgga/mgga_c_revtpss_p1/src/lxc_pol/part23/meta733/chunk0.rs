//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2504/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2504<F: Float>(t50298: F, t1565: F, t40781: F, t40488: F, t4354: F, t14862: F, t9775: F, t268: F, t40452: F, t4371: F, t2662: F, t40689: F, t4353: F) -> (F, F, F, F, F, F) {
    let t50299 = F::cast_from(0.16262400898971305032e-2_f64) * t50298;
    let t50370 = t40781 * t1565;
    let t50372 = t40488 * t4354;
    let t50374 = t9775 * t14862;
    let t50375 = F::cast_from(0.22866142996303859718e-3_f64) * t50374;
    let t50377 = t40452 * t4371 * t268;
    let t50381 = t40689 * t2662 * t4353 * t268;
    (t50299, t50370, t50372, t50375, t50377, t50381)
}
