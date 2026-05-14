//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 993/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk993<F: Float>(t40241: F, t10856: F, t7470: F, t10708: F, t7262: F, t3281: F, t10848: F, t11760: F, t2207: F, t3578: F, t494: F, t97: F, t113: F, t11505: F, t11588: F, t38355: F) -> (F, F, F, F, F, F, F, F) {
    let t40242 = 0.46574606203128791246e-1 * t40241;
    let t40243 = t10856 * t7470;
    let t40244 = 0.19514881078765566037e-1 * t40243;
    let t40251 = t10708 * t7262;
    let t40257 = t3281 * t7470;
    let t40258 = 0.10975748638225852664e-1 * t40257;
    let t40260 = t2207 * t11760 * t10848;
    let t40261 = 0.13972381860938637374e0 * t40260;
    let t40276 = t97 * t3578 * t494;
    let t40282 = t97 * t11505 * t113;
    let t40303 = t38355 * t11588;
    (t40242, t40244, t40251, t40258, t40261, t40276, t40282, t40303)
}
