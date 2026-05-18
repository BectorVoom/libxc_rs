//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 885/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk885<F: Float>(t1415: F, t1646: F, t42402: F, t12990: F, t7007: F, t42085: F, t550: F, t30733: F, t10122: F, t2464: F, t2465: F, t587: F) -> (F, F, F, F, F) {
    let t42405 = F::new(0.35750489951850426669e0) * t1415 * t42402 * t1646;
    let t42407 = F::new(0.71500979903700853338e0) * t12990 * t7007;
    let t42408 = t550 * t42085;
    let t42412 = t12990 * t30733;
    let t42413 = F::new(0.59584149919750711116e-1) * t42412;
    let t42416 = t587 * t2464 * t2465 * t10122;
    (t42405, t42407, t42408, t42413, t42416)
}
