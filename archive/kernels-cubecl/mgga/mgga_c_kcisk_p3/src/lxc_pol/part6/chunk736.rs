//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 736/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk736<F: Float>(t143: F, t3532: F, t1390: F, t213: F, t3830: F, t423: F, t394: F, t4143: F, t10471: F, t140: F, t416: F, t382: F) -> (F, F, F, F, F, F) {
    let t14093 = t143 * t3532;
    let t14100 = t213 * t1390;
    let t14140 = F::cast_from(1.0_f64) / t3830 / t423;
    let t14208 = t394 * t4143;
    let t14223 = t140 * t10471 * t416;
    let t14255 = t382 * t3532;
    (t14093, t14100, t14140, t14208, t14223, t14255)
}
