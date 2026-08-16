//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1134/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1134<F: Float>(t15983: F, t515: F, t7661: F, t16016: F, t6215: F, t6875: F, t2067: F, t2402: F, t6611: F, t835: F, t16024: F, t15685: F, t6230: F) -> (F, F, F, F, F, F, F, F) {
    let t20978 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t15983;
    let t20979 = t7661 * t515;
    let t20980 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t20979;
    let t20981 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t16016;
    let t20982 = t6875 * t6215;
    let t20983 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t20982;
    let t20985 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2402 * t2067;
    let t20987 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t6611 * t835;
    let t20988 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t16024;
    let t20990 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t15685 * t6230;
    (t20978, t20980, t20981, t20983, t20985, t20987, t20988, t20990)
}
