//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2139/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2139<F: Float>(t13716: F, t1450: F, t2014: F, t7237: F, t18163: F, t7735: F, t27137: F, t4254: F, t25082: F, t75353: F, t8717: F, t7311: F, t9593: F) -> (F, F, F, F, F) {
    let t98564 = t1450 * t13716;
    let t98567 = F::new(3.0) * t2014 * t7237 * t98564;
    let t98569 = F::new(2.0) * t18163 * t7735;
    let t98571 = F::new(4.0) * t4254 * t27137;
    let t98574 = F::new(6.0) * t25082 * t8717 * t75353;
    let t98575 = t7311 * t9593;
    (t98567, t98569, t98571, t98574, t98575)
}
