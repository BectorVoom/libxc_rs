//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 758/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk758<F: Float>(t2414: F, t539: F, t188: F, t4723: F, t4725: F, t6229: F, t6234: F, t6236: F, t6238: F, t6240: F, t6243: F, t6247: F, t6249: F, t6252: F, t6257: F, t6261: F, t6265: F) -> (F, F, F) {
    let t7179 = t2414 * t539;
    let t7180 = t7179 * t188;
    let t7182 = -t4723 + t4725 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7180 - t6229 - t6234 - t6236 - t6238 + t6240 + t6243 + t6247 + t6249 + t6252 - t6257 + t6261 + t6265;
    (t7179, t7180, t7182)
}
