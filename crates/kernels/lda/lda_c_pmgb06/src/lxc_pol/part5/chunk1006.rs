//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1006/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1006<F: Float>(t19791: F, t439: F, t5260: F, t1901: F, t19754: F, t2010: F, t2002: F, t6376: F, t6379: F, t6472: F, t6275: F, t6382: F, t1385: F, t2064: F, t2578: F, t161: F, t489: F, t7747: F) -> (F, F, F, F, F, F, F, F) {
    let t20813 = 32.0 / 27.0 * t439 * t5260 * t19791;
    let t20816 = 4.0 / 3.0 * t2010 * t1901 * t19754;
    let t20818 = 2.0 / 15.0 * t2002 * t6376;
    let t20820 = 2.0 / 5.0 * t2002 * t6379;
    let t20822 = 2.0 / 3.0 * t2002 * t6472;
    let t20824 = 8.0 / 15.0 * t6275 * t6382;
    let t20828 = t439 * t1385 * t2578 * t2064 / 15.0;
    let t20830 = t161 * t489 * t7747;
    (t20813, t20816, t20818, t20820, t20822, t20824, t20828, t20830)
}
