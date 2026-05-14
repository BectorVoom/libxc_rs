//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 997/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk997<F: Float>(t16042: F, t12047: F, t12051: F, t20978: F, t20980: F, t20981: F, t20983: F, t20985: F, t20987: F, t20988: F, t20990: F, t20992: F, t20993: F, t1318: F, t2526: F, t5269: F, t593: F, t811: F) -> (F, F, F) {
    let t20994 = 16.0 / 45.0 * t16042;
    let t20995 = -t20978 + t20980 + t20981 + t20983 - t20985 - t12047 + t12051 - t20987 - t20988 + t20990 + t20992 - t20993 + t20994;
    let t21001 = 8.0 / 5.0 * t1318 * t5269 * t2526 * t811 * t593;
    (t20994, t20995, t21001)
}
