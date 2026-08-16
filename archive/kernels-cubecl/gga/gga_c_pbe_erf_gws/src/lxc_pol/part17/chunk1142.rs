//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1142/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1142<F: Float>(t14504: F, t14527: F, t14553: F, t14574: F, t898: F, t338: F, t353: F, t1161: F, t3222: F, t13781: F, t3972: F, t1113: F, t9520: F, param_a_c: F) -> (F, F, F, F, F, F, F) {
    let t14576 = t14504 + t14527 + t14553 + t14574;
    let t14577 = t898 * t14576;
    let t14579 = t338 * t353 * t14577;
    let t14582 = t1161 * param_a_c;
    let t14583 = t14582 * t3222;
    let t14584 = t13781 * t14583;
    let t14585 = t3972 * t14584;
    let t14587 = t1113 * t9520;
    (t14576, t14577, t14579, t14582, t14584, t14585, t14587)
}
