//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1202/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1202(t1097: f64, t1884: f64, t22162: f64, t22166: f64, t22170: f64, t22179: f64, t222: f64, t22285: f64, t2635: f64, t2636: f64, t2674: f64, t2766: f64, t2770: f64, t2773: f64, t2783: f64, t2784: f64, t2787: f64, t2788: f64, t2792: f64, t2798: f64, t2802: f64, t2803: f64, t2806: f64, t2809: f64, t2810: f64, t470: f64, t567: f64, t7237: f64, t7241: f64, t7316: f64, t7317: f64, t7360: f64, t7367: f64, t7378: f64, t7410: f64, t7411: f64, t7420: f64, t7468: f64, t7469: f64, t7471: f64, t7472: f64, t7477: f64) -> f64 {
    let t22396 = -t22162 - t22166 - t22170 + t22179 - 0.24828486201251232145e5_f64 * t470 / t2787 / t2770 * t22285 * t7471 + 0.43374325201206959368e-1_f64 * t222 * t7367 * t2806 + 0.12842595503380418954e1_f64 * t222 * t1884 * t2635 * t2810 - 0.21687162600603479684e-1_f64 * t222 * t2798 * t7411 - 0.38025319932552508021e2_f64 * t222 * t567 * t7237 * t7317 + 0.13698666666666666666e0_f64 * t222 * t7378 * t2784 + 0.44060335298551228073e1_f64 * t222 * t1884 * t2788 * t2792 - 0.68493333333333333332e-1_f64 * t222 * t2766 * t7360 - 0.14171548179536397724e3_f64 * t222 * t567 * t7468 * t7472 + 0.21053605041484726346e2_f64 * t2809 * t2803 * t2674 + 0.61524113149298439947e4_f64 * t7316 * t2674 * t7241 * t2636 - 0.11579025239058625248e4_f64 * t7477 * t7420 * t2773 - 0.46785788981077169656e1_f64 * t2802 * t1097 * t7410 + 0.12414243100625616072e5_f64 * t7469 * t2783 * t7471 * t2773;
    t22396
}
