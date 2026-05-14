//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 976/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk976<F: Float>(t1447: F, t5487: F, t1423: F, t5483: F, t10079: F, t13231: F, t13233: F, t13236: F, t13238: F, t13240: F, t13242: F, t13244: F, t13246: F, t13248: F, t10087: F, t10089: F) -> (F, F, F, F, F) {
    let t13249 = t1447 * t5487;
    let t13250 = 4.0 / 45.0 * t13249;
    let t13251 = t1423 * t5483;
    let t13252 = 4.0 / 45.0 * t13251;
    let t13254 = t13231 + t13233 + t13236 + t13238 + t13240 + t13242 - t13244 - t13246 - t13248 - t13250 - t13252 - 8.0 / 135.0 * t10079;
    let t13257 = t10087 / 45.0;
    let t13258 = 2.0 / 45.0 * t10089;
    (t13250, t13252, t13254, t13257, t13258)
}
