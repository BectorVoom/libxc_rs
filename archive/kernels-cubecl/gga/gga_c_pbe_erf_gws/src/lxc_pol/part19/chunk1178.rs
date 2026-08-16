//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1178/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1178<F: Float>(t15204: F, t3983: F, t2503: F, t4127: F, t3863: F, t4039: F, t3788: F, t4023: F, t14015: F, t3754: F, t3749: F, t3783: F) -> (F, F, F, F, F, F, F) {
    let t15205 = t3983 * t15204;
    let t15216 = t4127 * t2503;
    let t15218 = t4039 * t3863;
    let t15220 = t3788 * t4023;
    let t15222 = t14015 * t3754;
    let t15224 = t4039 * t3749;
    let t15226 = t3783 * t4023;
    (t15205, t15216, t15218, t15220, t15222, t15224, t15226)
}
