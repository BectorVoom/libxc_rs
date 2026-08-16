//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1096/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1096<F: Float>(t13252: F, t9607: F, t4058: F, t945: F, t1172: F, t318: F, t2376: F, t4052: F, t829: F, t830: F) -> (F, F, F, F) {
    let t13544 = t9607 * t13252;
    let t13751 = t4058 * t945;
    let t13756 = t1172 * t318;
    let t13770 = t2376 * t4052;
    let t13772 = t829 * t830 * t13770;
    (t13544, t13751, t13756, t13772)
}
