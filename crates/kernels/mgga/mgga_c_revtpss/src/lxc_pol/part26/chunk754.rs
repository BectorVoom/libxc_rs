//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 754/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk754<F: Float>(t2619: F, t2622: F, t10552: F, t10554: F, t10557: F, t10560: F, t10562: F, t10564: F, t10566: F, t10568: F, t9333: F, t9394: F, t2390: F, t72: F, t757: F, t2629: F, t9863: F) -> (F, F, F, F) {
    let t10569 = t2622 * t2619;
    let t10570 = 0.73245789224026180216e-3 * t10569;
    let t10571 = t9333 - t10552 + t10554 + t10557 + t9394 + t10560 + t10562 + t10564 + t10566 - t10568 + t10570;
    let t10573 = t2390 * t72;
    let t10574 = t10573 * t757;
    let t10575 = 0.54934341918019635162e-3 * t10574;
    let t10577 = 0.16265371950452609763e-1 * t2629 * t9863;
    (t10570, t10571, t10575, t10577)
}
