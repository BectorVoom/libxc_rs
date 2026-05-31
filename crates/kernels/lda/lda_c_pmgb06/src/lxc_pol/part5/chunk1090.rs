//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1090/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1090<F: Float>(t20120: F, t16506: F, t16522: F, t16446: F, t183: F, t188: F, t19130: F, t20107: F, t20109: F, t20111: F, t20112: F, t20113: F, t20115: F, t20116: F) -> (F, F, F, F) {
    let t20121 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t20120;
    let t20122 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t16506;
    let t20123 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t16522;
    let t20124 = t20107 + t20109 + t20111 + t20112 + t20113 + F::cast_from(0.21642082724729686_f64) * t16446 - t20115 + t20116 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t19130 * t183 * t188 + t20121 + t20122 + t20123;
    (t20121, t20122, t20123, t20124)
}
