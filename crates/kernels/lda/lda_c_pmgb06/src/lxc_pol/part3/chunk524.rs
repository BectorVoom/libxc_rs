//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 524/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk524<F: Float>(t1241: F, t1249: F, t1302: F, t1303: F, t2185: F, t2188: F, t2191: F, t2212: F, t2222: F, t2245: F, t2247: F, t2248: F, t2249: F, t69: F) -> F {
    let t2255 = -t1241 + t2185 + t1249 + t2188 + t2191 - t2212 + t1302 + F::new(0.5747516666666667) * t1303 + F::new(0.5747516666666667) * t2245 + F::new(5.172765) * t2247 * t2248 * t2249 - F::new(1.724255) * t69 * t2222;
    t2255
}
