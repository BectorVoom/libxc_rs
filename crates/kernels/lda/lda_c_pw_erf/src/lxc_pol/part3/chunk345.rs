//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 345/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk345<F: Float>(t1246: F, t1268: F, t1252: F, t538: F, t1256: F, t1240: F, t1241: F, t1248: F, t1254: F, t1258: F, t1263: F, t1264: F, t25: F) -> (F, F, F, F) {
    let t1269 = t1268 * t1246;
    let t1272 = t538 * t1252;
    let t1275 = t538 * t1256;
    let t1278 = t1240 + 0.023994444444444443 * t1241 - 0.023994444444444443 * t1248 + 0.07198333333333333 * t1254 - 0.035991666666666665 * t1258 + t1263 + 0.008888888888888889 * t1264 - 0.0022222222222222222 * t25 * t1269 + 0.013333333333333334 * t25 * t1272 - 0.006666666666666667 * t25 * t1275;
    (t1269, t1272, t1275, t1278)
}
