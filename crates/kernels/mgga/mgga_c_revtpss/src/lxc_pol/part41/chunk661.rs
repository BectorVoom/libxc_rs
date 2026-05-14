//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 661/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk661<F: Float>(t1448: F, t1450: F, t565: F, t2219: F, t2223: F, t2226: F, t2230: F, t2233: F, t2239: F, t1466: F, t602: F, t1497: F, t644: F, t1469: F, t606: F) -> (F, F, F, F, F, F, F) {
    let t4140 = t1448 * t1450;
    let t4146 = t565 * t565;
    let t4147 = 1.0 / t4146;
    let t4171 = -t2219 + t2223 - t2226 + t2230 - t2233 + t2239;
    let t4173 = t1466 * t602;
    let t4178 = t1497 * t644;
    let t4181 = t606 * t1469;
    (t4140, t4146, t4147, t4171, t4173, t4178, t4181)
}
