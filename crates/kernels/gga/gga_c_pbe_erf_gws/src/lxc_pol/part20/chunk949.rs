//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 949/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk949<F: Float>(t337: F, t3791: F, t810: F, t2147: F, t3116: F, t3837: F, t6501: F, t11464: F, t254: F, t3223: F, t3765: F, t6402: F, t11539: F, t3816: F, t6627: F, t11806: F, t2170: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t11841 = t337 * t3791 * t810;
    let t11842 = t2147 * t11841;
    let t11844 = t3116 * t11842 / 48.0;
    let t11846 = t6501 * t3837;
    let t11848 = t254 * t11464;
    let t11849 = t11848 * t3223;
    let t11852 = t6402 * t3765;
    let t11854 = t11539 * t3223;
    let t11857 = t6627 * t3816;
    let t11860 = t2170 * t11806 * t875;
    (t11844, t11846, t11849, t11852, t11854, t11857, t11860)
}
