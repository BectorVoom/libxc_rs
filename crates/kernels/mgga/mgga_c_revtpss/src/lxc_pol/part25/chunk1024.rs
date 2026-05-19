//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1024/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1024<F: Float>(t3515: F, t3520: F, t5206: F, t1196: F, t1129: F, t3431: F, t408: F, t1149: F, t3385: F, t3434: F, t421: F, t1187: F, t3495: F) -> (F, F, F, F) {
    let t12222 = t3520 * t3515 * t5206;
    let t12224 = F::cast_from(0.51947577317044391277e2_f64) * t1196 * t12222;
    let t12226 = F::new(1.0) / t3431 / t1129;
    let t12227 = t408 * t12226;
    let t12228 = t3385 * t1149;
    let t12230 = F::new(1.0) / t3434 / t421;
    let t12231 = t12228 * t12230;
    let t12233 = F::cast_from(0.51726012919273400301e3_f64) * t12227 * t12231;
    let t12234 = t3495 * t1187;
    (t12224, t12228, t12233, t12234)
}
