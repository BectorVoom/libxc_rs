//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 839/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk839<F: Float>(t2268: F, t42212: F, t888: F, t39717: F, t12800: F, t6313: F, t6305: F, t2792: F, t3158: F, t12773: F, t12810: F, t10156: F, t2349: F, t2756: F, t3148: F, t12834: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t42678 = t2268 * t42212 * t888;
    let t42687 = 0.47425011059460249332e-2 * t39717;
    let t42689 = 0.26558006193297739625e0 * t6313 * t12800;
    let t42691 = 0.19918504644973304719e0 * t6305 * t12800;
    let t42694 = 0.19918504644973304719e0 * t2268 * t3158 * t2792;
    let t42695 = t6313 * t12773;
    let t42698 = 0.37940008847568199465e-1 * t6313 * t12810;
    let t42700 = t2268 * t10156 * t2349;
    let t42703 = 0.28455006635676149599e-1 * t6305 * t12810;
    let t42706 = 0.28455006635676149599e-1 * t2268 * t3148 * t2756;
    let t42708 = 0.37940008847568199465e-1 * t6313 * t12834;
    (t42678, t42687, t42689, t42691, t42694, t42695, t42698, t42700, t42703, t42706, t42708)
}
