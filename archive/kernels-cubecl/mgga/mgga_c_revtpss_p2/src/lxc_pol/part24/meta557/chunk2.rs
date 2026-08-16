//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1667/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1667<F: Float>(t88166: F, t88218: F, t88242: F, t88262: F, t41672: F, t77499: F, t77505: F, t77507: F, t77509: F, t77663: F, t77667: F, t88089: F, t88097: F, t88144: F, t88147: F, t88150: F, t88161: F, t88164: F) -> (F, F) {
    let t88264 = t88166 + t88218 + t88242 + t88262;
    let t88291 = -F::cast_from(0.10805407407407407407e0_f64) * t88144 - F::cast_from(0.104195e0_f64) * t88147 + F::cast_from(0.55570666666666666666e0_f64) * t88150 - F::cast_from(0.55570666666666666668e0_f64) * t77663 + F::cast_from(0.12349037037037037037e0_f64) * t77667 - F::cast_from(0.185931e2_f64) * t88089 + F::cast_from(0.41318e1_f64) * t88097 + t41672 + F::cast_from(0.76514814814814814814e0_f64) * t77499 + F::cast_from(0.68863333333333333332e0_f64) * t77505 - F::cast_from(0.27545333333333333332e1_f64) * t77507 + F::cast_from(0.41318e1_f64) * t77509 - F::cast_from(0.125034e1_f64) * t88161 - F::cast_from(0.104195e0_f64) * t88164;
    (t88264, t88291)
}
