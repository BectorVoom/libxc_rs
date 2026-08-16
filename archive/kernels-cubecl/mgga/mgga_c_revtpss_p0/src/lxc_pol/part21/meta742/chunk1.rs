//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2614/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2614<F: Float>(t13872: F, t221: F, t3978: F, t9921: F, t1320: F, t13632: F, t13672: F, t1317: F, t13680: F, t3860: F, t5567: F, t46960: F) -> (F, F, F, F, F, F) {
    let t48141 = t221 * t13872;
    let t48143 = t3978 * t9921 * t48141;
    let t48152 = t1320 * t13632;
    let t48153 = F::cast_from(12.0_f64) * t48152;
    let t48154 = t1320 * t13672;
    let t48155 = F::cast_from(12.0_f64) * t48154;
    let t48157 = F::cast_from(24.0_f64) * t1317 * t13680;
    let t48158 = t3860 * t5567;
    let t48159 = F::cast_from(36.0_f64) * t48158;
    let t48160 = F::cast_from(36.0_f64) * t46960;
    (t48143, t48153, t48155, t48157, t48159, t48160)
}
