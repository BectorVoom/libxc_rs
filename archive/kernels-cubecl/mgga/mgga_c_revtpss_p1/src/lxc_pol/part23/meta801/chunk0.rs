//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2628/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2628<F: Float>(t18657: F, t212: F, t689: F, t780: F, t252: F, t2769: F, t2782: F, t6071: F, t886: F, t4500: F, t51421: F, t14495: F, t14567: F) -> (F, F, F, F) {
    let t62549 = t689 * t212 * t18657 * t780;
    let t62572 = t2782 * t252 * t2769 * t6071 * t886;
    let t62577 = t51421 * t4500;
    let t62583 = t2782 * t14567 * t14495;
    (t62549, t62572, t62577, t62583)
}
