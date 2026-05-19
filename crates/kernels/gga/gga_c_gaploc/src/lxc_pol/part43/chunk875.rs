//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 875/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk875<F: Float>(t12797: F, t1358: F, t39717: F, t12800: F, t6313: F, t6305: F, t2268: F, t2792: F, t3158: F, t12810: F, t2756: F, t3148: F) -> (F, F, F, F, F, F, F, F) {
    let t42673 = t1358 * t12797;
    let t42674 = F::cast_from(0.31616674039640166221e-2_f64) * t42673;
    let t42687 = F::cast_from(0.47425011059460249332e-2_f64) * t39717;
    let t42689 = F::cast_from(0.26558006193297739625e0_f64) * t6313 * t12800;
    let t42691 = F::cast_from(0.19918504644973304719e0_f64) * t6305 * t12800;
    let t42694 = F::cast_from(0.19918504644973304719e0_f64) * t2268 * t3158 * t2792;
    let t42698 = F::cast_from(0.37940008847568199465e-1_f64) * t6313 * t12810;
    let t42703 = F::cast_from(0.28455006635676149599e-1_f64) * t6305 * t12810;
    let t42706 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t3148 * t2756;
    (t42674, t42687, t42689, t42691, t42694, t42698, t42703, t42706)
}
