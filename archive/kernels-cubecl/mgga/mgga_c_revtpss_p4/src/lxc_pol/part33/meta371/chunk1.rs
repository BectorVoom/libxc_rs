//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1407/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1407<F: Float>(t14671: F, t14686: F, t4366: F, t14931: F, t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F, t4469: F, t822: F) -> (F, F, F, F, F) {
    let t14933 = t14686 * t14671 * t4366;
    let t14934 = t14931 * t14933;
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    let t14951 = t874 * t4522 * t2470;
    let t14972 = t822 * t4469;
    (t14933, t14934, t14948, t14951, t14972)
}
