//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 443/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk443<F: Float>(t2209: F, t370: F, t1241: F, t1249: F, t1259: F, t1261: F, t1274: F, t1277: F, t1280: F, t2185: F, t2188: F, t2191: F, t2212: F, t2215: F, t2217: F, t2222: F, t2227: F, t2229: F, t342: F, t35: F, t360: F, t63: F) -> (F, F) {
    let t2233 = t370 * t2209;
    let t2236 = -t1241 + t2185 + t1249 + t2188 + t2191 - t2212 + t1259 + t1261 / 6.0 + t2215 / 6.0 + 3.0 / 2.0 * t360 * t35 * t2217 - t360 * t2222 / 2.0 + t1274 + 0.73452 * t1277 + t1280 + 0.73452 * t2227 + 5.87616 * t63 * t2229 * t342 - 1.46904 * t63 * t2233;
    (t2233, t2236)
}
