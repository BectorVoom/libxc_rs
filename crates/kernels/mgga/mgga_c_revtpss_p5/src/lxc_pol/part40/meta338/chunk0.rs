//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1128/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1128<F: Float>(t1466: F, t2246: F, t1497: F, t2248: F, t4241: F, t644: F, t2315: F, t10355: F, t1469: F, t2251: F, t2275: F, t4186: F) -> (F, F, F, F, F, F) {
    let t13272 = t1466 * t2246;
    let t13283 = t1497 * t2248;
    let t13286 = t4241 * t644;
    let t13289 = t1497 * t2315;
    let t13299 = t10355 * t1469 * t2251;
    let t13302 = t2275 * t4186;
    (t13272, t13283, t13286, t13289, t13299, t13302)
}
