//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1154/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1154<F: Float>(t14916: F, t829: F, t830: F, t1205: F, t2494: F, t2376: F, t2409: F, t1144: F, t338: F, t4111: F, t14611: F, t1161: F, t4110: F) -> (F, F, F, F, F, F) {
    let t14918 = t829 * t830 * t14916;
    let t14922 = t1205 * t2494;
    let t14924 = t2409 * t2376 * t14922;
    let t14928 = t338 * t1144 * t4111;
    let t14931 = F::new(7.0) / F::new(2304.0) * t14611;
    let t14935 = t4110 * t1161;
    (t14918, t14922, t14924, t14928, t14931, t14935)
}
