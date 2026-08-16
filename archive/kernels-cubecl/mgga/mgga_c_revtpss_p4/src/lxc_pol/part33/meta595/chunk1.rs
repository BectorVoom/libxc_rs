//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2014/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2014<F: Float>(t94589: F, t94590: F, t7289: F, t94377: F, t7285: F, t9288: F, t7284: F, t7243: F, t9292: F, t2453: F, t3908: F, t7275: F) -> (F, F, F, F, F, F) {
    let t94591 = t94589 * t94590;
    let t94593 = t7289 * t94377;
    let t94600 = t7285 * t9288;
    let t94602 = F::cast_from(0.22487184191643109717e-1_f64) * t7284 * t94600;
    let t94608 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t7243;
    let t94616 = t2453 * t7275 * t3908;
    (t94591, t94593, t94600, t94602, t94608, t94616)
}
