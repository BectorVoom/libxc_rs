//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 961/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk961<F: Float>(t6705: F, t815: F, t1874: F, t2592: F, t16442: F, t16444: F, t16448: F, t16455: F, t6626: F, t802: F, t16506: F, t16522: F, t16446: F, t183: F, t188: F, t19130: F, t20107: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20109 = t6705 * t815 / 10.0;
    let t20111 = t2592 * t1874 / 10.0;
    let t20112 = t16442 / 15.0;
    let t20113 = t16444 / 15.0;
    let t20115 = 2.0 / 15.0 * t16448;
    let t20116 = 2.0 / 15.0 * t16455;
    let t20120 = t802 * t6626;
    let t20121 = 2.0 / 15.0 * t20120;
    let t20122 = 4.0 / 135.0 * t16506;
    let t20123 = 16.0 / 81.0 * t16522;
    let t20124 = t20107 + t20109 + t20111 + t20112 + t20113 + 0.21642082724729686 * t16446 - t20115 + t20116 + 4.0 / 3.0 * t19130 * t183 * t188 + t20121 + t20122 + t20123;
    (t20109, t20111, t20112, t20113, t20115, t20116, t20121, t20122, t20123, t20124)
}
