//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 995/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk995<F: Float>(t10856: F, t7470: F, t38166: F, t10708: F, t7262: F, t3281: F, t10848: F, t11760: F, t2207: F, t3578: F, t494: F, t97: F, t113: F, t11505: F, t11588: F, t38355: F) -> (F, F, F, F, F, F, F, F) {
    let t40243 = t10856 * t7470;
    let t40248 = 0.84755945902752848174e0 * t38166;
    let t40251 = t10708 * t7262;
    let t40257 = t3281 * t7470;
    let t40260 = t2207 * t11760 * t10848;
    let t40276 = t97 * t3578 * t494;
    let t40282 = t97 * t11505 * t113;
    let t40303 = t38355 * t11588;
    (t40243, t40248, t40251, t40257, t40260, t40276, t40282, t40303)
}
