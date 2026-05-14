//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1325/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1325<F: Float>(t1327: F, t35869: F, t6204: F, t8048: F, t114243: F, t26814: F, t3482: F, t26818: F, t5633: F, t114378: F, t26035: F, t5874: F, t119130: F, t9446: F, t34758: F, t3748: F) -> (F, F, F, F, F, F) {
    let t119182 = t6204 * t35869 * t8048 * t1327;
    let t119186 = t3482 * t114243 * t26814;
    let t119189 = t5633 * t114243 * t26818;
    let t119194 = t114378 * t5874 * t26035;
    let t119197 = t9446 * t119130;
    let t119203 = t3748 * t34758;
    (t119182, t119186, t119189, t119194, t119197, t119203)
}
