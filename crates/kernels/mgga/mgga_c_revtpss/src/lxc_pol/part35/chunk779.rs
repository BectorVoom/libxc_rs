//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 779/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk779<F: Float>(t20849: F, t225: F, t480: F, t3520: F, t6534: F, t5265: F, t5274: F, t12916: F, t6689: F, t3718: F, t1219: F, t6667: F, t247: F, t3634: F, t6429: F, t1261: F) -> (F, F, F, F, F, F, F) {
    let t20850 = t20849 * t225;
    let t20851 = t20850 * t480;
    let t20895 = t3520 * t6534;
    let t20917 = t5274 * t5265;
    let t20926 = t12916 * t6689;
    let t20927 = t3718 * t20926;
    let t20966 = t6667 * t1219;
    let t20973 = t247 * t3634 * t6429;
    let t20974 = t1261 * t20973;
    (t20850, t20851, t20895, t20917, t20927, t20966, t20974)
}
