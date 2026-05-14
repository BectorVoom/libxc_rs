//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1060/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1060<F: Float>(t2146: F, t6362: F, t6367: F, t4763: F, t6371: F, t1325: F, t2328: F, t5289: F, t542: F, t806: F, t11983: F, t1318: F, t2466: F, t593: F, t811: F, t2065: F, t5269: F, t6242: F) -> (F, F, F, F, F, F) {
    let t22084 = 8.0 / 15.0 * t2146 * t6362;
    let t22086 = 4.0 / 9.0 * t2146 * t6367;
    let t22088 = 8.0 / 9.0 * t4763 * t6371;
    let t22093 = 8.0 / 5.0 * t1325 * t5289 * t2328 * t806 * t542;
    let t22098 = 24.0 / 5.0 * t1318 * t11983 * t2466 * t811 * t593;
    let t22102 = 16.0 / 5.0 * t1318 * t5269 * t6242 * t2065;
    (t22084, t22086, t22088, t22093, t22098, t22102)
}
