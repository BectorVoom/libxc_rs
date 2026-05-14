//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1179/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1179<F: Float>(t1464: F, t1900: F, t7149: F, t10491: F, t6217: F, t43917: F, t25409: F, t6963: F, t25462: F, t28987: F, t29006: F, t1091: F, t14686: F, t14690: F, t14906: F, t1506: F, t25393: F, t25456: F, t25459: F, t2665: F, t29008: F, t6216: F, t98283: F, t98297: F, t98306: F, t98309: F, t98370: F) -> (F, F) {
    let t111732 = t1464 * t7149 * t1900;
    let t111733 = t10491 * t6217;
    let t111737 = t43917 * t6217;
    let t111743 = 2.0 / 9.0 * t6963 * t25409;
    let t111747 = t25462 * t28987 / 27.0;
    let t111751 = 2.0 / 3.0 * t25462 * t29006;
    let t111754 = -t98283 / 9.0 - t29008 * t25456 / 27.0 - t6216 * t2665 * t98370 * t1091 / 18.0 - 4.0 / 9.0 * t111732 * t111733 * t14690 + 4.0 / 27.0 * t111732 * t111737 * t14686 - t14906 * t1506 + t111743 + t6963 * t25393 - 2.0 / 27.0 * t98297 + t111747 + 4.0 / 27.0 * t98306 + 4.0 / 27.0 * t98309 - t111751 + 2.0 * t25459 * t29006;
    (t111732, t111754)
}
