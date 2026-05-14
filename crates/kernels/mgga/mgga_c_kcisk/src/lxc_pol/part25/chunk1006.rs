//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1006/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1006<F: Float>(t17064: F, t740: F, t1950: F, t2576: F, t5336: F, t17056: F, t5317: F, t16980: F, t747: F, t746: F, t1948: F, t5278: F, t7406: F, t2567: F, t5285: F, t5284: F) -> (F, F, F, F, F, F, F) {
    let t17816 = t17064 * t740;
    let t17817 = t17816 * t1950;
    let t17819 = t2576 * t5336;
    let t17821 = t17056 * t740;
    let t17822 = t17821 * t5317;
    let t17824 = t747 * t16980;
    let t17825 = t746 * t17824;
    let t17826 = t1948 * t17825;
    let t17828 = t5278 * t7406;
    let t17830 = t2567 * t5285;
    let t17831 = t5284 * t17830;
    (t17817, t17819, t17822, t17825, t17826, t17828, t17831)
}
