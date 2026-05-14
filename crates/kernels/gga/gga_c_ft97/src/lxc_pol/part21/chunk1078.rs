//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1078/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1078<F: Float>(t26203: F, t8392: F, t26166: F, t47667: F, t26358: F, t26272: F, t26350: F, t1882: F, t26334: F, t47660: F, t5630: F, t100: F, t7241: F, t26195: F, t26454: F, t26416: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t103649 = 4.0 / 27.0 * t8392 * t26203;
    let t103654 = t47667 * t26166;
    let t103686 = 2.0 / 27.0 * t8392 * t26358;
    let t103695 = 2.0 / 27.0 * t8392 * t26272;
    let t103698 = 4.0 / 81.0 * t8392 * t26350;
    let t103745 = 2.0 / 27.0 * t1882 * t26334;
    let t103753 = t47660 * t5630;
    let t103761 = t7241 * t100 * t5630;
    let t103769 = 2.0 / 9.0 * t1882 * t26195;
    let t103783 = 2.0 / 9.0 * t1882 * t26454;
    let t103796 = 2.0 / 9.0 * t1882 * t26416;
    (t103649, t103654, t103686, t103695, t103698, t103745, t103753, t103761, t103769, t103783, t103796)
}
