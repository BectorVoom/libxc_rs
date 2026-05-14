//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1090/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1090<F: Float>(t4948: F, t831: F, t1499: F, t2625: F, t486: F, t6616: F, t12274: F, t12276: F, t12278: F, t12281: F, t132: F, t1547: F, t2583: F, t2470: F, t3223: F, t1447: F, t6120: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16294 = t831 * t4948;
    let t16295 = 4.0 / 45.0 * t16294;
    let t16297 = t1499 * t2625 / 30.0;
    let t16298 = t486 * t6616;
    let t16299 = 2.0 / 45.0 * t16298;
    let t16300 = 2.0 / 45.0 * t12274;
    let t16301 = 4.0 / 45.0 * t12276;
    let t16302 = 2.0 / 45.0 * t12278;
    let t16303 = 2.0 / 45.0 * t12281;
    let t16305 = t132 * t1547 * t2583;
    let t16306 = t16305 / 135.0;
    let t16307 = t3223 * t2470;
    let t16308 = 2.0 / 243.0 * t16307;
    let t16309 = t1447 * t6120;
    (t16295, t16297, t16299, t16300, t16301, t16302, t16303, t16306, t16308, t16309)
}
