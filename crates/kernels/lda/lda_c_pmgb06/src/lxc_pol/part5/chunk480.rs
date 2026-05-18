//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 480/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk480<F: Float>(t1067: F, t2149: F, t2152: F, t2154: F, t2156: F, t2161: F, t2395: F, t2396: F, t248: F, t283: F, t961: F, t970: F, t982: F) -> F {
    let t2405 = t248 * t2396 + F::new(0.0197516734986138) * t2395 * t283 - F::new(1.1696447245269292) * t2149 - F::new(8.0) * t2154 - F::new(8.0) * t2156 - F::new(0.0003662289461201309) * t2152 + F::new(2.0) * t2161 - t961 - t970 + t982 + t1067;
    t2405
}
