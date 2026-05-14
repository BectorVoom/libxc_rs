//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1044/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1044<F: Float>(t1143: F, t6126: F, t1144: F, t858: F, t2416: F, t3199: F, t326: F, t825: F, t6148: F, t3067: F, t830: F, t9550: F, t9607: F, t2494: F, t3222: F, t28667: F, t9370: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34963 = t1143 * t6126;
    let t35566 = t858 * t1144;
    let t36129 = t3199 * t2416;
    let t36199 = t326 * t825;
    let t36200 = t36199 * t6148;
    let t36201 = t830 * t3067;
    let t36865 = t9607 * t9550;
    let t36888 = t2494 * param_a_c;
    let t36889 = t36888 * t3222;
    let t37214 = t28667 * t9370;
    (t34963, t35566, t36129, t36199, t36200, t36201, t36865, t36889, t37214)
}
