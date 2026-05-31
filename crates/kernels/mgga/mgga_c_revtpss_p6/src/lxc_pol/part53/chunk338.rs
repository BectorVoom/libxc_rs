//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 338/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk338<F: Float>(t1469: F, t633: F, t637: F) -> F {
    let t1490 = t633 * t1469;
    let t1491 = t637 * t1469;
    let t1493 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1490 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1491;
    t1493
}
