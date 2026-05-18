//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 356/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk356<F: Float>(t1438: F, t88: F, t119: F, t331: F, t84: F, t465: F, t4: F, t60: F) -> (F, F, F, F) {
    let t1439 = t1438 * t88;
    let t1440 = F::new(32.0) * t1439;
    let t1444 = t119 * t331 * t84;
    let t1445 = t465 * t1444;
    let t1446 = F::new(0.24415406715670879921e-3) * t1445;
    let t1447 = t60 * t4;
    (t1440, t1444, t1446, t1447)
}
