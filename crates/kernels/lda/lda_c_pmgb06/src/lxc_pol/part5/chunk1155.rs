//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1155/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1155<F: Float>(t1988: F, t493: F, t6544: F, t1444: F, t7634: F, t1420: F, t7532: F, t12772: F, t2500: F, t439: F, t13709: F, t13714: F, t13720: F, t20863: F, t20866: F, t20870: F, t20872: F, t20874: F) -> (F, F, F, F, F) {
    let t20877 = t493 * t1988 * t6544 / F::new(15.0);
    let t20879 = t1444 * t7634 / F::new(9.0);
    let t20881 = F::new(2.0) / F::new(15.0) * t1420 * t7532;
    let t20884 = F::new(2.0) / F::new(15.0) * t439 * t12772 * t2500;
    let t20885 = t20863 + t20866 + t20870 - t13709 - t13714 - t13720 + t20872 + t20874 + t20877 + t20879 - t20881 - t20884;
    (t20877, t20879, t20881, t20884, t20885)
}
