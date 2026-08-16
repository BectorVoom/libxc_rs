//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 189/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk189(t673: f64, t702: f64, t140: f64, t479: f64, t709: f64, t725: f64, t716: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t728 = t673 * t702;
    let t732 = 0.619125e-2_f64 * t725 * t709 - 0.39796666666666666666e-1_f64 * t140 * t479 * t728;
    let t733 = t732 * t716;
    let t734 = t733 * sigma2;
    (t728, t732, t733, t734)
}
