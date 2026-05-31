//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3163/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3163<F: Float>(t12227: F, t12228: F, t5108: F, t3451: F, t5117: F, t3383: F, t5060: F, t3386: F, t12247: F, t1719: F, t12249: F, t1756: F, t3521: F) -> (F, F, F, F, F) {
    let t58333 = F::cast_from(0.57895126195293126241e3_f64) * t12227 * t5108 * t12228;
    let t58336 = t5117 * t3451;
    let t58339 = t5060 * t3383;
    let t58341 = F::cast_from(6.0_f64) * t58339 * t3386;
    let t58342 = t1719 * t12247;
    let t58344 = F::cast_from(0.96491876992155210402e2_f64) * t58342 * t12249;
    let t58345 = t3521 * t1756;
    (t58333, t58336, t58341, t58344, t58345)
}
