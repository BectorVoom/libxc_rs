//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1033/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1033<F: Float>(t1354: F, t2841: F, t4429: F, t118: F, t5567: F, t11676: F, t1366: F, t5652: F, t5655: F, t2349: F, t3309: F, t3333: F, t5649: F, t11751: F, t11756: F, t11758: F, t11759: F, t11760: F, t11761: F, t11763: F) -> (F, F, F, F) {
    let t14303 = t4429 * t2841 * t1354;
    let t14306 = t5567 * t118;
    let t14308 = t11676 * t118;
    let t14310 = t5652 * t1366;
    let t14311 = 0.21642082724729686 * t14310;
    let t14312 = t5655 * t1366;
    let t14314 = t2349 * t3309;
    let t14316 = t5649 * t3333;
    let t14318 = t14311 + 0.21642082724729686 * t14312 - 0.09618703433213194 * t14314 - 0.011181742741110338 * t14316 + t11751 - t11756 + t11758 + t11759 + t11760 + t11761 - t11763;
    (t14303, t14306, t14308, t14318)
}
