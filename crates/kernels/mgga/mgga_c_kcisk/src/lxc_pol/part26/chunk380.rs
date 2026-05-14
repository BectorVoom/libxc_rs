//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 380/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk380<F: Float>(t1359: F, t2077: F, t2084: F, t1355: F, t2083: F, t306: F) -> (F, F) {
    let t2188 = 0.1982e-1 * t2084 - t1359 - 0.41275e-2 * t2077;
    let t2191 = t1355 * t2083 / 4.0 + t306 * t2188 / 2.0;
    (t2188, t2191)
}
