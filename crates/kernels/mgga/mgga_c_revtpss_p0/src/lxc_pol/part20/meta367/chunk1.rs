//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1341/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1341<F: Float>(t10558: F, t72: F, t757: F, t10573: F, t2619: F, t2598: F, t9321: F, t760: F, t2523: F, t9387: F, t2495: F, t39875: F, t9367: F) -> (F, F, F, F, F, F) {
    let t40125 = t10558 * t72 * t757;
    let t40126 = F::cast_from(0.73245789224026180216e-3_f64) * t40125;
    let t40127 = t10573 * t2619;
    let t40128 = F::cast_from(0.14649157844805236043e-2_f64) * t40127;
    let t40129 = t9321 * t2598;
    let t40131 = F::cast_from(0.21053605041484726346e2_f64) * t760 * t40129;
    let t40132 = t2523 * t9387;
    let t40133 = F::cast_from(0.23392894490538584828e1_f64) * t40132;
    let t40135 = t9367 * t39875 * t2495;
    (t40126, t40128, t40129, t40131, t40133, t40135)
}
