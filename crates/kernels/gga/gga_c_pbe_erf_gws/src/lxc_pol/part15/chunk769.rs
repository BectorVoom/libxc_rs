//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 769/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk769<F: Float>(t6781: F, t938: F, t829: F, t830: F, t2074: F, t831: F, t2370: F, t4383: F, t824: F, t822: F) -> (F, F, F, F, F, F) {
    let t6782 = t6781 * t938;
    let t6784 = t829 * t830 * t6782;
    let t6787 = t831 * t2074;
    let t6789 = t2370 * t830 * t6787;
    let t6792 = t824 * t4383;
    let t6793 = t822 * t6792;
    (t6782, t6784, t6787, t6789, t6792, t6793)
}
