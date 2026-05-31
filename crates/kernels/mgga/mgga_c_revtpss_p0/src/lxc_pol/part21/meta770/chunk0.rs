//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2726/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2726<F: Float>(t50132: F, t50149: F, t10115: F, t1570: F, t11007: F, t1579: F, t252: F, t2771: F, t2782: F, t4322: F, t9292: F, t2772: F, t4321: F, t689: F) -> (F, F, F, F, F, F) {
    let t50151 = t50132 / F::cast_from(2.0_f64) + t50149 / F::cast_from(2.0_f64);
    let t50155 = t10115 * t1570;
    let t50161 = t11007 * t1579;
    let t50164 = t2782 * t252 * t50161 * t2771;
    let t50166 = t9292 * t4322;
    let t50169 = t689 * t4321 * t2772;
    (t50151, t50155, t50161, t50164, t50166, t50169)
}
