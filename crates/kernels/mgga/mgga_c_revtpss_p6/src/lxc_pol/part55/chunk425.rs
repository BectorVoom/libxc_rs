//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 425/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk425<F: Float>(t2113: F, t2115: F, t572: F, t573: F, t55: F, t61: F, t68: F, t72: F) -> (F, F, F) {
    let t2118 = t2113 * t573 + F::cast_from(3.0_f64) * t2115 * t572;
    let t2121 = t55 * t61 - t68;
    let t2122 = t2121 * t72;
    (t2118, t2121, t2122)
}
