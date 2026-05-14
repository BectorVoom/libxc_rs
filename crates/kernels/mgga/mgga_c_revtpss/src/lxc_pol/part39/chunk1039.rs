//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1039/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1039<F: Float>(t4171: F, t602: F, t1466: F, t2246: F, t1497: F, t2248: F, t4241: F, t644: F, t2315: F, t10355: F, t1469: F, t2251: F, t2275: F, t4186: F, t606: F, t2258: F, t4201: F) -> (F, F, F, F, F, F, F, F) {
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    let t13283 = t1497 * t2248;
    let t13286 = t4241 * t644;
    let t13289 = t1497 * t2315;
    let t13299 = t10355 * t1469 * t2251;
    let t13302 = t2275 * t4186;
    let t13303 = t13302 * t606;
    let t13306 = t4201 * t2258;
    (t13269, t13272, t13283, t13286, t13289, t13299, t13303, t13306)
}
