//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1191/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1191<F: Float>(t31837: F, t33695: F, t31841: F, t31838: F, t33715: F, t845: F, t126138: F, t2747: F, t31767: F, t31772: F, t2769: F, t34074: F) -> (F, F, F, F) {
    let t126213 = t33695 * t31837;
    let t126214 = t126213 * t31841;
    let t126226 = t31838 * t845 * t33715;
    let t126232 = t31767 * t2747 * t31772 * t126138;
    let t126250 = t34074 * t2769;
    (t126214, t126226, t126232, t126250)
}
