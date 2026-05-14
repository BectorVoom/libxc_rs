//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1055/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1055<F: Float>(t119781: F, t1579: F, t247: F, t837: F, t867: F, t125: F, t244: F, t246: F, t31838: F, t4533: F, t126273: F, t248: F, t8485: F, t31767: F, t31772: F, t4364: F) -> (F, F, F, F) {
    let t126340 = t119781 * t247 * t867 * t1579 * t837;
    let t126345 = t31838 * t244 * t246 * t125 * t4533;
    let t126358 = t126273 * t8485 * t248;
    let t126365 = t31767 * t4364 * t31772 * t4533;
    (t126340, t126345, t126358, t126365)
}
