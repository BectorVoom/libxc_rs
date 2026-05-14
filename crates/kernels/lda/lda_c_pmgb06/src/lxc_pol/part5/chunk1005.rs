//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1005/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1005<F: Float>(t20798: F, t161: F, t1639: F, t166: F, t7857: F, t486: F, t7859: F, t13440: F, t20784: F, t20786: F, t20789: F, t20791: F, t20792: F, t20794: F, t20797: F, t2960: F, t439: F, t477: F, t7481: F) -> (F, F, F, F, F) {
    let t20799 = 2.0 / 15.0 * t20798;
    let t20803 = t161 * t166 * t1639 * t7857 / 30.0;
    let t20805 = t486 * t7859 / 30.0;
    let t20806 = -t20784 - t20786 - t20789 - t20791 - t20792 - t20794 + t13440 + t20797 + t20799 - t20803 - t20805;
    let t20810 = 2.0 / 9.0 * t439 * t2960 * t7481 * t477;
    (t20799, t20803, t20805, t20806, t20810)
}
