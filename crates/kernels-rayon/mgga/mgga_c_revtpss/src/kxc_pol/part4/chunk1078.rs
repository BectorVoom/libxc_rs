//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1078/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1078(t12916: f64, t3722: f64, t3718: f64, t3172: f64, t3590: f64, t1247: f64, t3612: f64, t3610: f64, t1260: f64, t3666: f64, t3713: f64, t3711: f64) -> (f64, f64, f64, f64, f64) {
    let t12917 = t12916 * t3722;
    let t12918 = t3718 * t12917;
    let t12941 = t3172 * t3590;
    let t12942 = t1247 * t12941;
    let t12948 = t3172 * t3612;
    let t12949 = t3610 * t12948;
    let t12956 = t3666 * t1260;
    let t12959 = t3172 * t3713;
    let t12960 = t3711 * t12959;
    (t12918, t12942, t12949, t12956, t12960)
}
