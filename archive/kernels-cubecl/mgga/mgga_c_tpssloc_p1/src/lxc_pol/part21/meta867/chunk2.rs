//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3166/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3166<F: Float>(t11818: F, t1213: F, t248: F, t6219: F, t3036: F, t6163: F, t3500: F, t3503: F, t1210: F, t15734: F, t5005: F, t19047: F, t3572: F) -> (F, F, F, F, F) {
    let t65528 = t1213 * t248 * t11818 * t6219;
    let t65539 = t6163 * t3036;
    let t65541 = t3500 * t3503 * t65539;
    let t65545 = t3500 * t1210 * t65539;
    let t65552 = t5005 * t15734;
    let t65554 = t19047 * t3572;
    (t65528, t65541, t65545, t65552, t65554)
}
