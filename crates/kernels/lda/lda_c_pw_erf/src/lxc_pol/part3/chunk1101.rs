//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1101/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1101<F: Float>(t5064: F, t518: F, t2007: F, t12641: F, t2146: F, t3829: F, t3421: F, t4763: F, t3864: F, t3819: F, t4738: F, t3982: F) -> (F, F, F, F, F, F, F, F) {
    let t12881 = t5064 * t518;
    let t12883 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t12881 * t2007;
    let t12885 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12641 * t2007;
    let t12887 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2146 * t3829;
    let t12889 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4763 * t3421;
    let t12890 = t2146 * t3864;
    let t12891 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t12890;
    let t12893 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4738 * t3819;
    let t12895 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2146 * t3982;
    (t12881, t12883, t12885, t12887, t12889, t12891, t12893, t12895)
}
