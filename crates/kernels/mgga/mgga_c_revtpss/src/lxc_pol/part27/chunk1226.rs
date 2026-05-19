//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1226/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1226<F: Float>(t10510: F, t25399: F, t10115: F, t1951: F, t7058: F, t92871: F, t1032: F, t11007: F, t233: F, t25372: F, t10996: F, t25377: F) -> (F, F, F, F, F, F) {
    let t93273 = t25399 * t10510;
    let t93276 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t1951;
    let t93278 = F::cast_from(0.22487184191643109717e-1_f64) * t7058 * t92871;
    let t93279 = t1032 * t11007;
    let t93280 = t93279 * t233;
    let t93281 = t25372 * t93280;
    let t93282 = t25377 * t10996;
    (t93273, t93276, t93278, t93280, t93281, t93282)
}
