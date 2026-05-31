//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1357/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1357<F: Float>(t10281: F, t10288: F, t10290: F, t4171: F, t602: F, t1466: F, t2246: F) -> (F, F, F, F, F) {
    let t13264 = F::cast_from(80.0_f64) * t10281;
    let t13265 = F::cast_from(180.0_f64) * t10288;
    let t13266 = F::cast_from(252.0_f64) * t10290;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t13264, t13265, t13266, t13269, t13272)
}
