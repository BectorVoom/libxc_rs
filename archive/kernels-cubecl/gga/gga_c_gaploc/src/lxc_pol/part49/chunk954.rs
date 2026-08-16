//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 954/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk954<F: Float>(t12990: F, t7007: F, t30733: F, t10122: F, t2464: F, t2465: F, t587: F, t27003: F, t9438: F, t12965: F, t1407: F, t41634: F, t912: F) -> (F, F, F, F, F, F) {
    let t42407 = F::cast_from(0.71500979903700853338e0_f64) * t12990 * t7007;
    let t42412 = t12990 * t30733;
    let t42413 = F::cast_from(0.59584149919750711116e-1_f64) * t42412;
    let t42416 = t587 * t2464 * t2465 * t10122;
    let t42420 = t587 * t9438 * t27003;
    let t42421 = F::cast_from(0.31952438294933958064e-1_f64) * t42420;
    let t42422 = t1407 * t12965;
    let t42425 = t587 * t912 * t41634;
    (t42407, t42413, t42416, t42421, t42422, t42425)
}
