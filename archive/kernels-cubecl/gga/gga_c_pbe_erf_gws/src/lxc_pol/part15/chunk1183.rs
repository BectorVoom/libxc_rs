//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1183/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1183<F: Float>(t27047: F, t3067: F, t4016: F, t814: F, t13784: F, t13808: F, t1192: F, t19631: F, t829: F, t830: F, t2271: F, t332: F) -> (F, F, F, F) {
    let t50924 = t27047 * t3067 * t4016 * t814;
    let t50927 = t13808 * t13784;
    let t50930 = t19631 * t1192;
    let t50932 = t829 * t830 * t50930;
    let t50935 = t2271 * t332;
    (t50924, t50927, t50932, t50935)
}
