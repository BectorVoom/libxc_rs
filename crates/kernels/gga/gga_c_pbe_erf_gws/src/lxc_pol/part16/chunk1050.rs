//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1050/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1050<F: Float>(t1153: F, t9505: F, t6639: F, t9499: F, t3252: F, t3259: F, t810: F, t3258: F, t3257: F, t2118: F, t814: F, t821: F) -> (F, F, F, F, F, F, F) {
    let t9506 = t1153 * t9505;
    let t9509 = t9499 * t6639;
    let t9512 = t3252 * t9505;
    let t9515 = t3259 * t810;
    let t9516 = t3258 * t9515;
    let t9517 = t3257 * t9516;
    let t9520 = t2118 * t814;
    let t9521 = t821 * t9520;
    (t9506, t9509, t9512, t9516, t9517, t9520, t9521)
}
