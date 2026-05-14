//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 923/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk923<F: Float>(t1022: F, t2986: F, t1012: F, t2983: F, t400: F, t1023: F, t1054: F, t2946: F, t3111: F, t174: F, t3149: F, t998: F, t155: F, t3127: F, t3131: F, t3135: F, t3137: F) -> (F, F, F, F, F, F, F) {
    let t8370 = t2986 * t1022;
    let t8373 = 6152.338212604677 * t400 * t2983 * t1012 * t8370;
    let t8382 = 21.053604230838733 * t400 * t1054 * t1023;
    let t8386 = 623.3672123775311 * t400 * t2946 * t1012 * t3111;
    let t8389 = 0.07123333333333333 * t174 * t998 * t3149;
    let t8393 = 36.84545214203136 * t174 * t155 * t3127 * t3131;
    let t8397 = 6.873371715287382 * t174 * t155 * t3135 * t3137;
    (t8370, t8373, t8382, t8386, t8389, t8393, t8397)
}
