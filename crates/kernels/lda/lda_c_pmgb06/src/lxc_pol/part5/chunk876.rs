//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 876/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk876<F: Float>(t2477: F, t3213: F, t224: F, t6704: F, t2493: F, t1963: F, t5220: F, t2591: F, t607: F, t446: F, t490: F, t6688: F, t1504: F, t2563: F, t1366: F, t7193: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16314 = t3213 * t2477;
    let t16343 = t6704 * t224;
    let t16350 = t3213 * t2493;
    let t16380 = t5220 * t1963;
    let t16382 = t2591 * t607;
    let t16383 = t16382 * t446;
    let t16442 = t6688 * t490;
    let t16444 = t2563 * t1504;
    let t16446 = t7193 * t1366;
    (t16314, t16343, t16350, t16380, t16382, t16383, t16442, t16444, t16446)
}
