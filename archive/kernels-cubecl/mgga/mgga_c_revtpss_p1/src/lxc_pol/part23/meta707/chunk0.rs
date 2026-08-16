//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2460/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2460<F: Float>(t10174: F, t2453: F, t1420: F, t4075: F, t786: F, t1359: F, t39501: F, t10115: F, t555: F, t1445: F, t10165: F, t9664: F) -> (F, F, F, F, F, F) {
    let t47520 = t2453 * t10174;
    let t47530 = t786 * t1420 * t4075;
    let t47561 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1359;
    let t47567 = t10115 * t555;
    let t47568 = t47567 * t1445;
    let t47570 = t10165 * t9664;
    (t47520, t47530, t47561, t47567, t47568, t47570)
}
