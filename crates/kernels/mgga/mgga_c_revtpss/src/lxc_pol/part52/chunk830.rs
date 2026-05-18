//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 830/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk830<F: Float>(t3335: F, t389: F, t1077: F, t992: F, t1031: F, t4171: F, t602: F, t1466: F, t2246: F) -> (F, F, F, F, F, F) {
    let t11108 = F::new(1.0) / t3335 / t389;
    let t11119 = t1077 * t1077;
    let t11120 = F::new(1.0) / t11119;
    let t11198 = t992 * t992;
    let t11199 = F::new(1.0) / t11198;
    let t11238 = t1031 * t1031;
    let t11239 = F::new(1.0) / t11238;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t11108, t11120, t11199, t11239, t13269, t13272)
}
