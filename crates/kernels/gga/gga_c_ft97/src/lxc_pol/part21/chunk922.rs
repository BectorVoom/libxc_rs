//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 922/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk922<F: Float>(t147: F, t26558: F, t26595: F, t26808: F, t27433: F, t184: F, t5: F, t6731: F, t1080: F, t21: F, t24157: F, t363: F, t3660: F, t3665: F, t3668: F, t3674: F, t3678: F, t5982: F, t5985: F, t650: F, t6732: F, t920: F) -> (F, F, F, F) {
    let t148 = 10000000.0 <= t147;
    let t27435 = t26558 + t26595 + t26808 + t27433;
    let t27436 = t27435 * t184;
    let t27440 = t5 * t6731;
    let t27462 = piecewise3(t148, 0.0, t5 * t27436 * t21 / 4.0 + t27440 * t650 / 4.0 + t5 * t6732 * t363 / 4.0 + t24157 * t1080 / 4.0 + t5985 * t3660 / 4.0 + t5985 * t3665 / 4.0 + t5985 * t3668 / 4.0 + t5 * t5982 * t920 / 4.0 + t5985 * t3674 / 4.0 + t5985 * t3678 / 2.0);
    (t27435, t27436, t27440, t27462)
}
