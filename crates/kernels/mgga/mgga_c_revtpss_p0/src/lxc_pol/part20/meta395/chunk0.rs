//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1451/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1451<F: Float>(t41491: F, t983: F, t11502: F, t11601: F, t981: F, t2922: F, t275: F, t2875: F, t2925: F, t11506: F, t15542: F, t3006: F) -> (F, F, F, F, F) {
    let t41493 = F::cast_from(0.23392894490538584828e1_f64) * t41491 * t983;
    let t41496 = F::cast_from(0.46785788981077169656e1_f64) * t981 * t11601 * t11502;
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    let t41500 = t2875 * t2875;
    let t41501 = t2925 * t2925;
    let t41502 = F::new(1.0) / t41501;
    let t41505 = F::cast_from(0.24955700379505800916e5_f64) * t41499 * t41500 * t41502;
    let t41509 = F::cast_from(0.61524113149298439947e4_f64) * t981 * t11506 * t3006 * t15542;
    (t41493, t41496, t41500, t41505, t41509)
}
