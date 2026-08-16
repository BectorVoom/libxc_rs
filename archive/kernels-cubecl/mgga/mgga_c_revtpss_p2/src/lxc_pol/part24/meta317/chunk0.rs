//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1104/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1104<F: Float>(t1357: F, t6919: F, t689: F, t1904: F, t5599: F, t212: F, t6888: F, t1358: F, t6896: F, t6895: F, t72: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22409 = t1357 * t6919;
    let t22410 = t689 * t22409;
    let t22427 = t5599 * t1904;
    let t22428 = t689 * t22427;
    let t22445 = t212 * t6888;
    let t22446 = t22445 * t1358;
    let t22447 = t689 * t22446;
    let t22449 = t1357 * t6896;
    let t22450 = t689 * t22449;
    let t22452 = t6895 * t72;
    let t22453 = t22452 * t686;
    (t22409, t22410, t22427, t22428, t22445, t22446, t22447, t22449, t22450, t22452, t22453)
}
