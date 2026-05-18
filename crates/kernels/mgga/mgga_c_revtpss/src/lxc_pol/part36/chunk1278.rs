//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1278/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1278<F: Float>(t30016: F, t686: F, t72: F, t94674: F, t94669: F, t1358: F, t212: F, t30055: F, t689: F, t30056: F, t7289: F, t7284: F) -> (F, F, F, F, F) {
    let t108293 = t30016 * t72 * t686;
    let t108294 = t94674 * t108293;
    let t108296 = t94669 * t108293;
    let t108302 = t689 * t212 * t30055 * t1358;
    let t108307 = t30056 * t72 * t686;
    let t108308 = t7289 * t108307;
    let t108332 = t7284 * t108307;
    (t108294, t108296, t108302, t108308, t108332)
}
