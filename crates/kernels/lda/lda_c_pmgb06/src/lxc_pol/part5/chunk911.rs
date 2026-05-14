//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 911/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk911<F: Float>(t15472: F, t6688: F, t853: F, t1447: F, t7671: F, t1423: F, t7667: F, t7640: F, t19224: F, t19227: F, t19231: F, t19233: F, t19236: F, t19237: F, t19238: F, t2470: F, t5194: F) -> (F, F, F, F, F, F, F) {
    let t19239 = 2.0 / 15.0 * t15472;
    let t19241 = t6688 * t853 / 10.0;
    let t19242 = t1447 * t7671;
    let t19243 = 4.0 / 45.0 * t19242;
    let t19244 = t1423 * t7667;
    let t19245 = 4.0 / 45.0 * t19244;
    let t19246 = t1447 * t7640;
    let t19247 = 2.0 / 135.0 * t19246;
    let t19248 = -t19224 - t19227 - t19231 - t19233 + t19236 - t19237 + t19238 - t19239 - t19241 + t19243 + t19245 + t19247;
    let t19251 = t5194 * t2470;
    (t19239, t19241, t19243, t19245, t19247, t19248, t19251)
}
