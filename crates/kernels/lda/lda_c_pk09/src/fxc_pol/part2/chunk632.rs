//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 632/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk632<F: Float>(t1284: F, t5253: F, t5012: F, t1468: F, t364: F, t1319: F, t4998: F, t363: F, t309: F, t5009: F, t372: F, t4993: F) -> (F, F, F, F, F, F) {
    let t5254 = t5253 * t1284;
    let t5256 = F::new(37.27051603526593) * t5254 * t5012;
    let t5257 = t364 * t1468;
    let t5258 = t5257 * t1284;
    let t5260 = F::new(9.87466743489671) * t5258 * t5012;
    let t5262 = F::new(3.2915558116322368) * t1319 * t4998;
    let t5266 = t363 * t363;
    let t5267 = F::new(1.0) / t5266;
    let t5272 = t5009 * t309;
    let t5273 = t372 * t4993;
    (t5256, t5260, t5262, t5267, t5272, t5273)
}
