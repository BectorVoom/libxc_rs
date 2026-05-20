//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2208/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2208<F: Float>(t4757: F, t906: F, t3092: F, t380: F, t994: F, t16088: F) -> (F, F, F, F) {
    let t16090 = t4757 * t906;
    let t16091 = t3092 * t16090;
    let t16094 = t994 * t380;
    let t16095 = t16094 * t16088;
    (t16090, t16091, t16094, t16095)
}
