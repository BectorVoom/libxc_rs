//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2003/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2003<F: Float>(t25516: F, t3278: F, t11223: F, t1976: F, t27639: F, t995: F, t19482: F, t988: F, t25610: F, t25604: F, t7156: F, t3268: F, t7143: F) -> (F, F, F, F, F, F, F) {
    let t93821 = t3278 * t25516;
    let t93884 = t11223 * t1976;
    let t93890 = t995 * t27639;
    let t93893 = t19482 * t988;
    let t93897 = t25610 * t27639;
    let t93904 = t7156 * t25604;
    let t93920 = t7143 * t3268;
    (t93821, t93884, t93890, t93893, t93897, t93904, t93920)
}
