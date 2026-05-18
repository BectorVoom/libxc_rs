//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 972/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk972<F: Float>(t13924: F, t2162: F, t571: F, t9432: F, t1351: F, t4574: F, t3975: F, t1518: F, t185: F, t2099: F, t4500: F, t784: F) -> (F, F, F, F, F, F) {
    let t13925 = F::new(8.0) / F::new(45.0) * t13924;
    let t13929 = t571 * t9432 * t2162;
    let t13930 = F::new(8.0) / F::new(45.0) * t13929;
    let t13962 = t4574 * t1351;
    let t13966 = t3975 * t1351;
    let t14004 = t185 * t1518 * t2099;
    let t14005 = F::new(4.0) / F::new(45.0) * t14004;
    let t14014 = t4500 * t784;
    (t13925, t13930, t13962, t13966, t14005, t14014)
}
