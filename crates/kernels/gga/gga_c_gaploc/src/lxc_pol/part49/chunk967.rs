//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 967/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk967<F: Float>(t42644: F, t30204: F, t31769: F, t9074: F, t10177: F, t19531: F, t883: F, t1358: F, t23915: F, t42195: F, t3394: F, t488: F, t9060: F) -> (F, F, F, F, F) {
    let t42645 = F::cast_from(0.16598753870811087267e-1_f64) * t42644;
    let t42647 = t9074 * t30204 * t31769;
    let t42648 = F::cast_from(0.284550066356761496e-1_f64) * t42647;
    let t42651 = t9074 * t19531 * t883 * t10177;
    let t42652 = F::cast_from(0.142275033178380748e-1_f64) * t42651;
    let t42655 = F::cast_from(0.18970004423784099732e-1_f64) * t1358 * t23915 * t42195;
    let t42659 = F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t9060 * t3394 * t488;
    (t42645, t42648, t42652, t42655, t42659)
}
