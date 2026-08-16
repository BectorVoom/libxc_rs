//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 786/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk786<F: Float>(t916: F, t9386: F, t128: F, t6: F, t442: F, t919: F, t1081: F, t2645: F, t7451: F, t8673: F, t6182: F, t8676: F) -> (F, F, F, F, F, F) {
    let t9387 = t916 * t9386;
    let t9388 = t6 * t128;
    let t9389 = t9388 * t442;
    let t9390 = t919 * t9389;
    let t9391 = t9387 * t9390;
    let t9393 = t1081 * t2645;
    let t9395 = t7451 * t8673;
    let t9396 = t8676 * t6182;
    (t9387, t9388, t9391, t9393, t9395, t9396)
}
