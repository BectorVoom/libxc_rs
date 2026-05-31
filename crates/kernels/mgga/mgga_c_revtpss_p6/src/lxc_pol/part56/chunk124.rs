//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 124/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk124<F: Float>(t473: F, t51: F, t52: F, rho1: F, sigma2: F) -> (F, F, F, F) {
    let t474 = sigma2 * sigma2;
    let t475 = t473 * t474;
    let t476 = t51 * t51;
    let t477 = t476 * rho1;
    let t479 = F::cast_from(1.0_f64) / t52 / t477;
    (t474, t475, t476, t479)
}
