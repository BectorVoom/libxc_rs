//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1138/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1138(t26843: f64, t3594: f64, t3598: f64, t7616: f64, t479: f64, t3089: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t26844 = t3594 * t26843;
    let t26848 = t7616 * t3598;
    let t26849 = t3594 * t26848;
    let t26865 = sigma2 * t479;
    let t26866 = t26865 * t3089;
    (t26844, t26848, t26849, t26865, t26866)
}
