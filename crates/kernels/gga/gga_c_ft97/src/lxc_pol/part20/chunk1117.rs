//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1117/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1117<F: Float>(t108179: F, t108187: F, t3699: F, t96934: F, t109397: F, t109400: F, t109404: F, t109409: F, t109417: F, t109421: F, t97123: F, t97381: F, t97384: F, t97385: F, t97391: F, t109322: F, t27: F, t676: F, t89: F) -> (F, F, F) {
    let t109425 = t96934 * t108187 * t3699 * t108179;
    let t109427 = t109397 - 3.0 / 4.0 * t109400 - 8.0 / 9.0 * t109404 + 3.0 / 2.0 * t109409 + 8.0 / 9.0 * t97123 + 2.0 * t109417 + t109421 / 3.0 + t109425 / 3.0 - t97381 - t97384 + t97385 + t97391;
    let t109431 = t89 * t27 * t676 * t109322;
    (t109425, t109427, t109431)
}
