//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1142/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1142<F: Float>(t109356: F, t109358: F, t109361: F, t109367: F, t109372: F, t109375: F, t109379: F, t109382: F, t109385: F, t109388: F, t109393: F, t109397: F, t109400: F, t109404: F, t109409: F, t109417: F, t109421: F, t109425: F, t97123: F, t97144: F, t97154: F, t97156: F, t97176: F) -> (F, F) {
    let t110201 = t109356 / 12.0;
    let t110202 = t109358 / 9.0;
    let t110211 = t110201 + t110202 + 11.0 / 27.0 * t109361 + 5.0 / 16.0 * t109367 + t109372 / 4.0 + t109375 / 27.0 + 5.0 / 81.0 * t109379 - 4.0 / 27.0 * t109382 - t109385 / 9.0 - 2.0 / 27.0 * t109388 + t109393;
    let t110224 = t109397 / 3.0 - t109400 / 4.0 - 8.0 / 27.0 * t109404 + t109409 / 2.0 + 8.0 / 27.0 * t97123 + 2.0 / 3.0 * t109417 + t109421 / 9.0 + t109425 / 9.0 - t97144 / 9.0 - 2.0 / 9.0 * t97154 + 4.0 / 27.0 * t97156 + t97176 / 24.0;
    (t110211, t110224)
}
