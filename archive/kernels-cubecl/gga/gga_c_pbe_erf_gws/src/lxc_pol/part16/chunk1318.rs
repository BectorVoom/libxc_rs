//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1318/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1318<F: Float>(t27047: F, t3067: F, t4216: F, t814: F, t1205: F, t26654: F, t829: F, t830: F, t4083: F, t8746: F, t2416: F, t4227: F) -> (F, F, F, F) {
    let t55137 = t27047 * t3067 * t4216 * t814;
    let t55140 = t26654 * t1205;
    let t55142 = t829 * t830 * t55140;
    let t55145 = t8746 * t4083;
    let t55151 = t2416 * t4227;
    (t55137, t55142, t55145, t55151)
}
