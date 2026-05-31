//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1150/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1150<F: Float>(t2002: F, t6472: F, t6275: F, t6382: F, t1385: F, t2064: F, t2578: F, t439: F, t161: F, t489: F, t7747: F, t2015: F, t2592: F) -> (F, F, F, F, F) {
    let t20822 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2002 * t6472;
    let t20824 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t6275 * t6382;
    let t20828 = t439 * t1385 * t2578 * t2064 / F::cast_from(15.0_f64);
    let t20830 = t161 * t489 * t7747;
    let t20831 = t20830 / F::cast_from(15.0_f64);
    let t20832 = t2592 * t2015;
    (t20822, t20824, t20828, t20831, t20832)
}
