//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 701/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk701<F: Float>(t5269: F, t6243: F, t1318: F, t2035: F, t4763: F, t2011: F, t2146: F, t2014: F, t2018: F, t2419: F, t549: F, t1319: F) -> (F, F, F, F, F, F, F, F) {
    let t6244 = t5269 * t6243;
    let t6246 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t6244;
    let t6248 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t4763 * t2035;
    let t6250 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2146 * t2011;
    let t6252 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2146 * t2014;
    let t6254 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2146 * t2018;
    let t6255 = t2419 * t549;
    let t6256 = t1319 * t6255;
    (t6244, t6246, t6248, t6250, t6252, t6254, t6255, t6256)
}
