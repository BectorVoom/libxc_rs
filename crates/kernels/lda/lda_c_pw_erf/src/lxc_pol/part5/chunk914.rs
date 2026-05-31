//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 914/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk914<F: Float>(t174: F, t205: F, t9810: F, t1332: F, t1350: F, t1953: F, t560: F, t4048: F, t56: F, t9812: F, t155: F, t188: F) -> (F, F, F, F, F, F, F) {
    let t10042 = t174 * t9810 * t205;
    let t10043 = F::cast_from(0.01959135802469136_f64) * t10042;
    let t10056 = F::cast_from(1.0_f64) / t1350 / t1332;
    let t10090 = t1953 * t560;
    let t10102 = t56 * t4048;
    let t10145 = F::cast_from(0.01959135802469136_f64) * t9812;
    let t10162 = t155 * t188;
    (t10042, t10043, t10056, t10090, t10102, t10145, t10162)
}
