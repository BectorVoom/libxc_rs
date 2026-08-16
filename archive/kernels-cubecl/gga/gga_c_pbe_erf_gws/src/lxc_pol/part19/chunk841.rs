//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 841/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk841<F: Float>(t542: F, t974: F, t496: F, t2900: F, t513: F, t1576: F, t981: F, t1563: F, t9: F, t155: F, t506: F, t2911: F, t2913: F) -> (F, F, F, F, F, F) {
    let t8199 = t542 * t974;
    let t8200 = t496 * t8199;
    let t8206 = t2900 * t513;
    let t8209 = t981 * t1576;
    let t8231 = t9 * t1563;
    let t8236 = t155 * t506;
    let t8238 = t2911 * t8236 * t2913;
    (t8199, t8200, t8206, t8209, t8231, t8238)
}
