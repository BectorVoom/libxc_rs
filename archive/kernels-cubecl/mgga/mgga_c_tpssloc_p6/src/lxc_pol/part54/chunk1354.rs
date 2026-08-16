//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1354/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1354<F: Float>(t115925: F, t25971: F, t24987: F, t8644: F, t101138: F, t26161: F, t31775: F, t1441: F, t6534: F, t2040: F, t33211: F, t7050: F) -> (F, F, F, F, F, F) {
    let t120899 = F::cast_from(3.0_f64) * t115925 * t25971;
    let t120900 = t24987 * t8644;
    let t120907 = F::cast_from(2.0_f64) * t26161 * t101138 * t31775;
    let t120908 = t1441 * t6534;
    let t120910 = F::cast_from(2.0_f64) * t120908 * t2040;
    let t120912 = F::cast_from(2.0_f64) * t33211 * t7050;
    (t120899, t120900, t120907, t120908, t120910, t120912)
}
