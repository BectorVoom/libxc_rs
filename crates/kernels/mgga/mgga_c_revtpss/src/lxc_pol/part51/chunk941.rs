//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 941/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk941<F: Float>(t373: F, t988: F, t372: F, t371: F, t31902: F, t378: F, t31991: F) -> (F, F, F, F) {
    let t32004 = t373 * t988;
    let t32005 = t372 * t32004;
    let t32006 = t371 * t32005;
    let t32009 = t31902 * t378;
    let t32010 = t32009 * t31991;
    (t32004, t32006, t32009, t32010)
}
