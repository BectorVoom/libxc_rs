//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2057/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2057<F: Float>(t1032: F, t3727: F, t2148: F, t1276: F, t3140: F, t26894: F, t26921: F, t1294: F, t471: F, t355: F, t1204: F, t7627: F) -> (F, F, F, F, F, F, F) {
    let t96873 = t3727 * t1032;
    let t96874 = t2148 * t96873;
    let t96910 = t2148 * t3727 * t3140 * t1276;
    let t96927 = t26894 * t26921;
    let t96928 = t471 * t1294;
    let t96929 = t355 * t96928;
    let t96933 = t1204 * t7627;
    (t96873, t96874, t96910, t96927, t96928, t96929, t96933)
}
