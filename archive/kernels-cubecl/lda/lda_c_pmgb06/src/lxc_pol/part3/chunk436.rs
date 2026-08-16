//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 436/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk436<F: Float>(t1493: F, t1501: F, t1503: F, t1506: F, t1514: F, t1516: F, t1519: F, t1544: F, t1546: F, t1550: F, t1553: F, t1557: F) -> F {
    let t1707 = t1493 + t1501 + t1503 + t1506 + t1514 + t1516 + t1519 + t1544 + t1546 - t1550 + t1553 - t1557;
    t1707
}
