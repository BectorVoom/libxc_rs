//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 648/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk648<F: Float>(t1983: F, t6268: F, t5187: F, t806: F, t2002: F, t2007: F, t1980: F, t801: F) -> (F, F, F, F) {
    let t6270 = 4.0 / 45.0 * t6268 * t1983;
    let t6272 = 2.0 / 45.0 * t5187 * t806;
    let t6274 = 2.0 / 45.0 * t2002 * t2007;
    let t6275 = t801 * t1980;
    (t6270, t6272, t6274, t6275)
}
