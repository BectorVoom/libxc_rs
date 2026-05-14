//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1312/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1312<F: Float>(t1253: F, t668: F, t111807: F, t125658: F, t125663: F, t125665: F, t125668: F, t125670: F, t125682: F, t125684: F, t2404: F, t25412: F, t2665: F, t28940: F, t28946: F, t28950: F, t31686: F, t4255: F, t6216: F, t683: F, t684: F, t7022: F, t98318: F) -> (F,) {
    let t125686 = t1253 * t668;
    let t125694 = 2.0 / 9.0 * t6216 * t25412 * t31686 * t684 - t6216 * t2665 * t125658 * t684 / 9.0 + 8.0 * t125663 + 8.0 * t125665 + 8.0 * t125668 - 4.0 * t125670 - 2.0 / 81.0 * t98318 + 2.0 / 9.0 * t6216 * t683 * t7022 * t28940 - 2.0 / 27.0 * t6216 * t2404 * t7022 * t28946 + 2.0 / 9.0 * t125682 - 2.0 * t125684 + 2.0 / 9.0 * t6216 * t25412 * t125686 * t4255 + 2.0 / 9.0 * t6216 * t111807 * t28950;
    (t125694,)
}
