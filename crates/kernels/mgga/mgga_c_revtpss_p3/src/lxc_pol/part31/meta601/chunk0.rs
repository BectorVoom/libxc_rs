//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2033/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2033<F: Float>(t94589: F, t97814: F, t2435: F, t27965: F, t14090: F, t26054: F, t25894: F, t97676: F, t97680: F, t14110: F, t94901: F, t10073: F, t1903: F, t2029: F, t25929: F) -> (F, F, F, F, F, F) {
    let t97815 = t94589 * t97814;
    let t97823 = t2435 * t27965;
    let t97825 = t26054 * t14090;
    let t97838 = F::cast_from(0.28912093960683998208e-1_f64) * t25894 * t97676 * t97680;
    let t97843 = t94901 * t14110;
    let t97847 = t10073 * t25929 * t2029 * t1903;
    (t97815, t97823, t97825, t97838, t97843, t97847)
}
