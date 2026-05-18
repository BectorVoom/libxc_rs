//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 874/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk874<F: Float>(t453: F, t6189: F, t1830: F, t473: F, t6185: F, t6160: F, t1619: F, t6165: F, t2571: F, t350: F, t2575: F, t2579: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6190 = t453 * t6189;
    let t6191 = t1830 * t6190;
    let t6193 = t473 * t6185;
    let t6196 = t473 * t6189;
    let t6199 = t473 * t6160;
    let t6202 = t1619 * t6165;
    let t6205 = t350 * t2571;
    let t6207 = t350 * t2575;
    let t6209 = t350 * t2579;
    (t6190, t6191, t6193, t6196, t6199, t6202, t6205, t6207, t6209)
}
