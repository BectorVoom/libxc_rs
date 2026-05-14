//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1132/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1132<F: Float>(t1097: F, t1884: F, t22162: F, t22166: F, t22170: F, t22179: F, t222: F, t22285: F, t2635: F, t2636: F, t2674: F, t2766: F, t2770: F, t2773: F, t2783: F, t2784: F, t2787: F, t2788: F, t2792: F, t2798: F, t2802: F, t2803: F, t2806: F, t2809: F, t2810: F, t470: F, t567: F, t7237: F, t7241: F, t7316: F, t7317: F, t7360: F, t7367: F, t7378: F, t7410: F, t7411: F, t7420: F, t7468: F, t7469: F, t7471: F, t7472: F, t7477: F) -> (F,) {
    let t22396 = -t22162 - t22166 - t22170 + t22179 - 0.24828486201251232145e5 * t470 / t2787 / t2770 * t22285 * t7471 + 0.43374325201206959368e-1 * t222 * t7367 * t2806 + 0.12842595503380418954e1 * t222 * t1884 * t2635 * t2810 - 0.21687162600603479684e-1 * t222 * t2798 * t7411 - 0.38025319932552508021e2 * t222 * t567 * t7237 * t7317 + 0.13698666666666666666e0 * t222 * t7378 * t2784 + 0.44060335298551228073e1 * t222 * t1884 * t2788 * t2792 - 0.68493333333333333332e-1 * t222 * t2766 * t7360 - 0.14171548179536397724e3 * t222 * t567 * t7468 * t7472 + 0.21053605041484726346e2 * t2809 * t2803 * t2674 + 0.61524113149298439947e4 * t7316 * t2674 * t7241 * t2636 - 0.11579025239058625248e4 * t7477 * t7420 * t2773 - 0.46785788981077169656e1 * t2802 * t1097 * t7410 + 0.12414243100625616072e5 * t7469 * t2783 * t7471 * t2773;
    (t22396,)
}
