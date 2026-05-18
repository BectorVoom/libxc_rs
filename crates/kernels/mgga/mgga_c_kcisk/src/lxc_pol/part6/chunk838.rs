//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 838/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk838<F: Float>(t1899: F, t28256: F, t5062: F, t1869: F, t2469: F, t8851: F, t11200: F, t2527: F, t5185: F, t7718: F, t5184: F, t5182: F) -> (F, F, F) {
    let t28257 = t1899 * t28256;
    let t28258 = t5062 * t28257;
    let t28259 = t1869 * t28258;
    let t28261 = t8851 * t2469;
    let t28262 = t28261 * t11200;
    let t28269 = t5185 * t7718 * t2527;
    let t28270 = t5184 * t28269;
    let t28271 = t5182 * t28270;
    (t28259, t28262, t28271)
}
