//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1050/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1050<F: Float>(t2719: F, t820: F, t844: F, t4368: F, t2482: F, t814: F, t14671: F, t14686: F, t4366: F, t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F) -> (F, F, F, F) {
    let t14923 = t820 * t2719 * t844;
    let t14925 = 0.40015750243531754508e-2 * t14923 * t4368;
    let t14931 = t2482 * t2719 * t814;
    let t14933 = t14686 * t14671 * t4366;
    let t14934 = t14931 * t14933;
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    let t14951 = t874 * t4522 * t2470;
    (t14925, t14934, t14948, t14951)
}
