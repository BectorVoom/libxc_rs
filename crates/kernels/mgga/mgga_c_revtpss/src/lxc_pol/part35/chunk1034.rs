//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1034/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1034<F: Float>(t109403: F, t94674: F, t30295: F, t686: F, t72: F, t7284: F, t30282: F, t25895: F, t689: F, t6919: F, t7492: F, t30266: F, t25904: F, t25878: F, t94669: F, t30308: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t109404 = t94674 * t109403;
    let t109407 = t30295 * t72 * t686;
    let t109408 = t7284 * t109407;
    let t109412 = t30282 * t72 * t686;
    let t109413 = t25895 * t109412;
    let t109417 = t689 * t7492 * t6919;
    let t109425 = t30266 * t689;
    let t109426 = t25904 * t109425;
    let t109434 = t25878 * t109412;
    let t109437 = t94669 * t109403;
    let t109449 = t30308 * t72 * t686;
    (t109404, t109407, t109408, t109413, t109417, t109425, t109426, t109434, t109437, t109449)
}
