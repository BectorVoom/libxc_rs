//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1081/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1081<F: Float>(t591: F, t7975: F, t13440: F, t13444: F, t13447: F, t13450: F, t20784: F, t20786: F, t20789: F, t20791: F, t20792: F, t20794: F, t20797: F, t20799: F, t20803: F, t20805: F, t20810: F, t20813: F, t20816: F, t20818: F, t20820: F, t20822: F, t20824: F, t20828: F) -> (F, F) {
    let t22018 = t7975 * t591;
    let t22021 = -t20784 - t20786 - t20789 - t20791 - t20792 - t20794 + t13440 + 2.0 / 9.0 * t22018 + t13444 + t13447 + 0.547 * t13450 + t20797;
    let t22024 = t20799 - t20803 - t20805 + t20810 - t20813 - t20816 + t20818 + t20820 - t20822 - t20824 - t20828;
    (t22021, t22024)
}
