//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 967/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk967<F: Float>(t3443: F, t1803: F, t185: F, t186: F, t22968: F, t3399: F, t3445: F, t22982: F, t22986: F, t22988: F, t22994: F, t47527: F, t47528: F, t47529: F, t47530: F, t40039: F) -> (F, F, F, F, F) {
    let t47531 = t3443 * t3443;
    let t47535 = 4.0 / 5.0 * t185 * t186 * t1803 * t47531;
    let t47536 = 64.0 / 405.0 * t22968;
    let t47538 = 8.0 / 5.0 * t3399 * t3445;
    let t47543 = t47527 - t47528 + t47529 + t47530 + t47535 - t47536 - t47538 + 0.60617527037037037035e-2 * t22982 - 8.0 / 9.0 * t22986 - 0.5402469135802469136e-1 * t22988 + 8.0 / 3.0 * t22994;
    let t47545 = 32.0 / 15.0 * t40039;
    (t47535, t47536, t47538, t47543, t47545)
}
