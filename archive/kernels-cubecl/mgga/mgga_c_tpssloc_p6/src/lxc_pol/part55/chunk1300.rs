//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1300/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1300<F: Float>(t2169: F, t8119: F, t1851: F, t8927: F, t34401: F, t576: F, t112: F, t34385: F, t118354: F, t120786: F, t120788: F, t120792: F, t120800: F, t120803: F, t120807: F, t123272: F, t123274: F, t123282: F, t123285: F, t123287: F, t123290: F, t1458: F, t31284: F, t33195: F, t671: F, t8508: F) -> (F, F, F, F) {
    let t125982 = t2169 * t8119;
    let t125988 = t1851 * t8927;
    let t125991 = t576 * t34401;
    let t126000 = t34385 * t112;
    let t126004 = t31284 + F::cast_from(0.135e2_f64) * t118354 * t1458 + t8508 + t120786 + F::cast_from(27.0_f64) * t123272 + F::cast_from(27.0_f64) * t123274 + t120788 + t33195 + t120792 + F::cast_from(27.0_f64) * t123282 + t120800 + t120803 + F::cast_from(54.0_f64) * t123285 + F::cast_from(54.0_f64) * t123287 + t120807 + F::cast_from(0.135e2_f64) * t126000 * t671 + F::cast_from(54.0_f64) * t123290;
    (t125982, t125988, t125991, t126004)
}
