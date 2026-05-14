//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1047/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1047<F: Float>(t3788: F, t4023: F, t14015: F, t3754: F, t3749: F, t4039: F, t3783: F, t14570: F, t3123: F, t14007: F, t3759: F, t14035: F, t3837: F, t3827: F, t4043: F, t3820: F, t4028: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15220 = t3788 * t4023;
    let t15222 = t14015 * t3754;
    let t15224 = t4039 * t3749;
    let t15226 = t3783 * t4023;
    let t15228 = t3123 * t14570;
    let t15230 = t14007 * t3759;
    let t15232 = t14035 * t3837;
    let t15234 = t4043 * t3827;
    let t15236 = t4028 * t3820;
    (t15220, t15222, t15224, t15226, t15228, t15230, t15232, t15234, t15236)
}
