//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1772/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1772<F: Float>(t45106: F, t45107: F, t89824: F, t89832: F, t90402: F, t90405: F, t90408: F, t90411: F, t90414: F, t90417: F, t90420: F, t90423: F, t90451: F, t90453: F) -> F {
    let t90701 = -F::new(0.104195e0) * t90402 + F::new(0.62517e0) * t90405 - F::new(0.125034e1) * t90408 + F::new(0.250068e1) * t90411 + F::new(0.104195e0) * t90414 - F::cast_from(0.10805407407407407407e0_f64) * t90417 - F::new(0.52945875e1) * t90420 + F::cast_from(0.2366859375e0_f64) * t90423 - F::cast_from(0.15302962962962962963e1_f64) * t89832 + t45106 + t45107 + F::new(0.6311625e0) * t90451 - F::cast_from(0.6618234375e1_f64) * t90453 + F::cast_from(0.34431666666666666667e1_f64) * t89824;
    t90701
}
