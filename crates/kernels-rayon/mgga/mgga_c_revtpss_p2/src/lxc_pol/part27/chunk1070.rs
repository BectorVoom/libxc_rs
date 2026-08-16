//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1070/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1070(t12845: f64, t12929: f64, t13005: f64, t13105: f64, t489: f64, t1269: f64, t3601: f64, t3769: f64, t1248: f64, t1287: f64, t3727: f64, t3584: f64, t3759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13107 = t12845 + t12929 + t13005 + t13105;
    let t13108 = t489 * t13107;
    let t13111 = t1269 * t3601;
    let t13112 = t13111 * t3769;
    let t13118 = t3727 * t1248 * t1287;
    let t13121 = t3759 * t3584;
    (t13107, t13108, t13111, t13112, t13118, t13121)
}
