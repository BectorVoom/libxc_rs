//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1231/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1231<F: Float>(t35084: F, t9679: F, t1799: F, t33017: F, t8480: F, t8485: F, t2464: F, t2469: F, t32935: F, t7261: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35085 = t9679 * t35084;
    let t35086 = t1799 * t35085;
    let t35089 = t33017 * t8480;
    let t35090 = t1799 * t35089;
    let t35092 = t9679 * t8485;
    let t35093 = t1799 * t35092;
    let t35095 = t2464 * t2469;
    let t35096 = t32935 * t35095;
    let t35097 = t7261 * t35096;
    (t35085, t35086, t35089, t35090, t35092, t35093, t35095, t35096, t35097)
}
