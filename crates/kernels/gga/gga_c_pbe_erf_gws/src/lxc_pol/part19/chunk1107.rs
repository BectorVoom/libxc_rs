//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1107/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1107<F: Float>(t20154: F, t2376: F, t4207: F, t814: F, t14327: F, t3083: F, t53353: F, t27047: F, t3067: F, t4216: F, t1205: F, t26654: F, t829: F, t830: F, t4083: F, t8746: F) -> (F, F, F, F, F, F) {
    let t55110 = t20154 * t2376 * t4207 * t814;
    let t55114 = 7.0 / 144.0 * t3083 * t14327;
    let t55117 = 7.0 / 144.0 * t53353;
    let t55137 = t27047 * t3067 * t4216 * t814;
    let t55140 = t26654 * t1205;
    let t55142 = t829 * t830 * t55140;
    let t55145 = t8746 * t4083;
    (t55110, t55114, t55117, t55137, t55142, t55145)
}
