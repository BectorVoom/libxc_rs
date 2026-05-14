//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 708/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk708<F: Float>(t342: F, t630: F, t7302: F, t5842: F, t72: F, t1349: F, t1526: F, t1527: F, t2: F, t32658: F, t32663: F, t32665: F, t32670: F, t343: F, t5917: F, t5922: F, t7298: F, t7299: F) -> (F, F, F) {
    let t32675 = t342 * t630 * t7302 / 12.0;
    let t32679 = t72 * t5842;
    let t32684 = (-t32658 * t7299 / 6.0 + t32663 + t1349 * t32665 / 18.0 + t1349 * t5922 / 3.0 - t7298 * t32670 / 6.0 - t32675 - t1526 * t1527 * t5917 / 12.0 - t342 * t343 * t32679 / 4.0) * t2;
    (t32675, t32679, t32684)
}
