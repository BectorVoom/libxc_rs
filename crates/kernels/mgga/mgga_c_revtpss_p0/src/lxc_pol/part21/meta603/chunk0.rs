//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2330/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2330<F: Float>(t2710: F, t2793: F, t39494: F, t2804: F, t874: F, t9288: F, t10535: F, t231: F, t2645: F, t281: F, t68: F, t211: F, t9644: F) -> (F, F, F, F) {
    let t39633 = F::cast_from(0.20561456923286030469e-1_f64) * t2710 * t2793 * t39494;
    let t39635 = t874 * t2804 * t9288;
    let t39640 = t10535 * t281 * t68 * t2645 * t231;
    let t39643 = F::new(1.0) / t9644 / t211;
    (t39633, t39635, t39640, t39643)
}
