//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2069/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2069<F: Float>(t25301: F, t99257: F, t25410: F, t7774: F, t93240: F, t1032: F, t4469: F, t867: F, t786: F, t7060: F, t7760: F, t2467: F) -> (F, F, F, F, F, F, F) {
    let t99258 = t99257 * t25301;
    let t99261 = t93240 * t25410 * t7774;
    let t99270 = t4469 * t1032;
    let t99271 = t99270 * t867;
    let t99272 = t786 * t99271;
    let t99274 = F::cast_from(0.14456046980341999104e-1_f64) * t99272 * t7060;
    let t99285 = t786 * t7760 * t867;
    let t99287 = F::cast_from(0.19514881078765566038e-1_f64) * t99285 * t2467;
    (t99258, t99261, t99270, t99271, t99274, t99285, t99287)
}
