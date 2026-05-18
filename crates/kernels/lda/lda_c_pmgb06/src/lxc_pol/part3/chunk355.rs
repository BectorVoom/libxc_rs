//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 355/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk355<F: Float>(t410: F, t56: F, t69: F, t1260: F, t1241: F, t1247: F, t1249: F, t1252: F, t1255: F, t1264: F, t1268: F) -> (F, F, F) {
    let t1302 = F::new(0.3831677777777778) * t69 * t410 * t56;
    let t1303 = t69 * t1260;
    let t1309 = -t1241 + t1247 + t1249 + t1252 - t1255 + t1302 + F::new(1.1495033333333333) * t1303 + F::new(5.172765) * t69 * t1264 - F::new(1.724255) * t69 * t1268;
    (t1302, t1303, t1309)
}
