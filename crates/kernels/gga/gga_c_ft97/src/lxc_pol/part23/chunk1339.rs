//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1339/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1339<F: Float>(t10683: F, t19362: F, t6317: F, t6318: F, t25165: F, t2665: F, t4969: F, t2862: F, t28735: F, t5337: F, t824: F, t70038: F, t840: F, t19399: F, t44280: F, t19409: F) -> (F, F, F, F, F, F) {
    let t126740 = t6317 * t10683 * t6318 * t19362;
    let t126744 = t6317 * t2665 * t25165 * t4969;
    let t126749 = t28735 * t2862 * t6318 * t5337 * t824;
    let t126753 = t28735 * t840 * t6318 * t70038;
    let t126757 = t6317 * t44280 * t6318 * t19399;
    let t126761 = t6317 * t10683 * t6318 * t19409;
    (t126740, t126744, t126749, t126753, t126757, t126761)
}
