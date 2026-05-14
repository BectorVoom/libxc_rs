//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1014/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1014<F: Float>(t139312: F, t139991: F, t139992: F, t148381: F, t148385: F, t148388: F, t148392: F, t148396: F, t148401: F, t148405: F, t148410: F, t148414: F, t148419: F, t148422: F, t148426: F, t148430: F) -> (F,) {
    let t148781 = -3.0 / 8.0 * t148381 - 3.0 * t148385 - t148388 / 2.0 + 2.0 * t148392 + 4.0 * t148396 - t139312 / 3.0 + 3.0 / 4.0 * t148401 + t139991 - t139992 - 8.0 / 3.0 * t148405 + 4.0 / 3.0 * t148410 - 4.0 / 3.0 * t148414 + 2.0 / 9.0 * t148419 + 4.0 / 3.0 * t148422 - 6.0 * t148426 + t148430 / 6.0;
    (t148781,)
}
