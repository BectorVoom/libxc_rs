//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1014/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1014<F: Float>(t4022: F, t863: F, t6523: F, t8867: F, t1150: F, t14028: F, t14042: F, t14047: F, t14529: F, t14531: F, t14533: F, t14536: F, t14539: F, t14542: F, t14544: F, t14046: F, t4171: F) -> (F, F, F, F) {
    let t14547 = t863 * t4022;
    let t14548 = t6523 * t8867;
    let t14549 = t14547 * t14548;
    let t14551 = t14028 * t1150;
    let t14553 = -t14529 / 768.0 - t14531 / 192.0 - t14533 / 48.0 - t14536 / 48.0 - t14539 / 96.0 + 7.0 / 144.0 * t14042 - t14542 / 48.0 + t14544 / 768.0 + 7.0 / 288.0 * t14047 + t14549 / 16.0 - 7.0 / 1152.0 * t14551;
    let t14554 = t14046 * t4171;
    (t14547, t14548, t14553, t14554)
}
