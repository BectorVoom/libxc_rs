//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 832/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk832<F: Float>(t2763: F, t327: F, t4043: F, t7191: F, t9700: F, t277: F, t8754: F, t5312: F, t3708: F, t7418: F, t8986: F, t961: F) -> (F, F, F, F, F, F) {
    let t9703 = t4043 * t327 * t2763 * t7191;
    let t9704 = t9700 * t9703;
    let t9706 = t277 * t8754;
    let t9707 = t9706 * t9703;
    let t9709 = t277 * t5312;
    let t9710 = t3708 * t7418;
    let t9711 = t9709 * t9710;
    let t9713 = t8986 * t961;
    (t9703, t9704, t9707, t9709, t9711, t9713)
}
