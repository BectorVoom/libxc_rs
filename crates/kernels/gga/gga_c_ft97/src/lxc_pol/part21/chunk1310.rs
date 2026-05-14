//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1310/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1310<F: Float>(t30540: F, t8392: F, t1882: F, t30508: F, t30401: F, t119556: F, t119560: F, t119565: F, t119569: F, t119573: F, t119575: F, t119579: F, t119583: F, t119586: F, t119590: F, t119594: F) -> (F, F, F, F) {
    let t120850 = t8392 * t30540;
    let t120858 = t1882 * t30508;
    let t120860 = t8392 * t30401;
    let t120891 = t119556 + 4.0 / 27.0 * t119560 - t119565 / 54.0 - 4.0 / 9.0 * t119569 - 2.0 / 9.0 * t119573 + t119575 / 54.0 + 2.0 / 9.0 * t119579 + t119583 / 3.0 - 4.0 / 9.0 * t119586 - t119590 / 18.0 + t119594 / 3.0;
    (t120850, t120858, t120860, t120891)
}
