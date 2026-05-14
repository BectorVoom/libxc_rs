//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1057/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1057<F: Float>(t2250: F, t51213: F, t2259: F, t14006: F, t6684: F, t14046: F, t14049: F, t14079: F, t2293: F, t816: F, t837: F, t2080: F, t2084: F, t833: F, t13800: F, t13972: F) -> (F, F, F, F, F, F, F) {
    let t51465 = t2250 * t51213;
    let t51466 = t51465 * t2259;
    let t51470 = t6684 * t14006;
    let t51473 = t14046 * t14049;
    let t51479 = t14079 * t2293;
    let t51502 = t816 * t837;
    let t51505 = t2080 * t2084 * t51502 * t833;
    let t51507 = t13972 * t13800;
    (t51465, t51466, t51470, t51473, t51479, t51505, t51507)
}
