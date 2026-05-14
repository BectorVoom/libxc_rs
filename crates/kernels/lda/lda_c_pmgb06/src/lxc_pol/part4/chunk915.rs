//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 915/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk915<F: Float>(t132: F, t1541: F, t1547: F, t1710: F, t485: F, t500: F, t1451: F, t3223: F, t1498: F, t607: F, t1489: F, t1600: F, t1387: F, t3213: F, t1683: F, t1730: F) -> (F, F, F, F, F, F, F, F) {
    let t9259 = t132 * t1547 * t1541;
    let t9266 = t485 * t1710;
    let t9267 = t9266 * t500;
    let t9269 = t3223 * t1451;
    let t9271 = t1498 * t607;
    let t9321 = t1489 * t1600;
    let t9330 = t3213 * t1387;
    let t9338 = t1683 * t1730;
    (t9259, t9266, t9267, t9269, t9271, t9321, t9330, t9338)
}
