//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1308/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1308<F: Float>(t11336: F, t2512: F, t2516: F, t4287: F, t2520: F, t1404: F, t27010: F, t23302: F, t4293: F, t1003: F, t11346: F, t11348: F, t23058: F, t260: F, t2605: F, t31763: F, t31767: F, t31769: F, t31914: F, t31916: F, t31919: F, t31929: F) -> (F, F, F, F, F) {
    let t31933 = 1.0 * t11336 * t2512;
    let t31934 = t4287 * t2516;
    let t31936 = 0.16081979498692535067e2 * t31934 * t2520;
    let t31938 = 2.0 * t27010 * t1404;
    let t31940 = 2.0 * t23302 * t4293;
    let t31941 = -0.10254018858216406658e4 * t1003 * t11346 * t23058 + t31763 - 0.20508037716432813316e4 * t2605 * t11348 + t31767 + t31769 + t31914 + t31916 + t31919 + 0.19751673498613801407e-1 * t260 * t31929 + t31933 + t31936 + t31938 - t31940;
    (t31933, t31936, t31938, t31940, t31941)
}
