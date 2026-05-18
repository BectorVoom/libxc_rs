//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 895/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk895<F: Float>(t2409: F, t2501: F, t3189: F, t3744: F, t4414: F, t2366: F, t3916: F, t833: F, t3039: F, t3920: F, t3909: F, t840: F) -> (F, F, F, F, F, F) {
    let t9948 = t2409 * t2501 * t3189;
    let t9953 = t4414 * t3744;
    let t9955 = t3916 * t2366;
    let t9956 = t9955 * t833;
    let t9958 = t3039 * t3920;
    let t9962 = t840 * t3909;
    (t9948, t9953, t9955, t9956, t9958, t9962)
}
