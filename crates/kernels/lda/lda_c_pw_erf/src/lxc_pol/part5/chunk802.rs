//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 802/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk802<F: Float>(t1062: F, t22: F, t19: F, t301: F, t305: F, t732: F, t8359: F, t1022: F, t2986: F, t1012: F, t2983: F, t400: F, t1023: F, t1054: F, t2946: F, t3111: F) -> (F, F, F, F, F, F) {
    let t8363 = 1.0 / t22 / t1062;
    let t8368 = 0.3407285805772476 * t305 / t8359 * t8363 * t301 * t19 * t732;
    let t8370 = t2986 * t1022;
    let t8373 = 6152.338212604677 * t400 * t2983 * t1012 * t8370;
    let t8382 = 21.053604230838733 * t400 * t1054 * t1023;
    let t8386 = 623.3672123775311 * t400 * t2946 * t1012 * t3111;
    (t8363, t8368, t8370, t8373, t8382, t8386)
}
