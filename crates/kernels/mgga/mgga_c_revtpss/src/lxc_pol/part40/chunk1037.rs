//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1037/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1037<F: Float>(t12916: F, t3722: F, t3718: F, t3172: F, t3590: F, t1247: F, t3612: F, t3610: F, t1260: F, t3666: F, t3713: F, t3711: F, t127: F, t3661: F, t371: F, t1235: F) -> (F, F, F, F, F, F) {
    let t12917 = t12916 * t3722;
    let t12918 = t3718 * t12917;
    let t12941 = t3172 * t3590;
    let t12942 = t1247 * t12941;
    let t12948 = t3172 * t3612;
    let t12949 = t3610 * t12948;
    let t12956 = t3666 * t1260;
    let t12959 = t3172 * t3713;
    let t12960 = t3711 * t12959;
    let t12963 = t371 * t127 * t3661;
    let t12964 = t1235 * t12963;
    (t12918, t12942, t12949, t12956, t12960, t12964)
}
