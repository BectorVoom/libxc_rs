//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1152/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1152<F: Float>(t24: F, t10523: F, t10528: F, t1430: F, t18408: F, t2179: F, t28895: F, t28898: F, t28906: F, t3019: F, t507: F, t7932: F, t7935: F, t821: F, t8742: F, t9784: F, t204: F, t205: F, zeta_threshold: F) -> (F, F) {
    let t90 = t24 <= zeta_threshold;
    let t31086 = piecewise3(t90, 0.0, 280.0 / 81.0 * t18408 * t10523 * t507 + 56.0 / 9.0 * t9784 * t1430 - 28.0 / 9.0 * t7932 * t28895 - 8.0 / 3.0 * t7935 * t28898 + 4.0 / 3.0 * t3019 * t8742 + 4.0 / 9.0 * t2179 * t10528 * t507 - t821 * t28906 / 3.0);
    let t31088 = t204 * t205 * t31086;
    (t31086, t31088)
}
