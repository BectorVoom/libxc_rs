//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1135/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1135<F: Float>(t2146: F, t6272: F, t16036: F, t16042: F, t12047: F, t12051: F, t20978: F, t20980: F, t20981: F, t20983: F, t20985: F, t20987: F, t20988: F, t20990: F) -> (F, F, F, F) {
    let t20992 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2146 * t6272;
    let t20993 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t16036;
    let t20994 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t16042;
    let t20995 = -t20978 + t20980 + t20981 + t20983 - t20985 - t12047 + t12051 - t20987 - t20988 + t20990 + t20992 - t20993 + t20994;
    (t20992, t20993, t20994, t20995)
}
