//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 759/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk759<F: Float>(t42644: F, t30204: F, t31769: F, t9074: F, t10177: F, t19531: F, t883: F, t1358: F, t23915: F, t42195: F, t3394: F, t488: F, t9060: F, t12797: F, t39717: F, t12800: F, t6313: F) -> (F, F, F, F, F, F, F, F) {
    let t42645 = 0.16598753870811087267e-1 * t42644;
    let t42647 = t9074 * t30204 * t31769;
    let t42648 = 0.284550066356761496e-1 * t42647;
    let t42651 = t9074 * t19531 * t883 * t10177;
    let t42652 = 0.142275033178380748e-1 * t42651;
    let t42655 = 0.18970004423784099732e-1 * t1358 * t23915 * t42195;
    let t42659 = 0.31616674039640166221e-2 * t1358 * t9060 * t3394 * t488;
    let t42673 = t1358 * t12797;
    let t42674 = 0.31616674039640166221e-2 * t42673;
    let t42687 = 0.47425011059460249332e-2 * t39717;
    let t42689 = 0.26558006193297739625e0 * t6313 * t12800;
    (t42645, t42648, t42652, t42655, t42659, t42674, t42687, t42689)
}
