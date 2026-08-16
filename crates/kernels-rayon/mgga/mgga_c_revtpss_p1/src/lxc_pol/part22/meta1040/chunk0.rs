//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3632/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3632(t12227: f64, t3385: f64, t6474: f64, t16942: f64, t1733: f64, t3384: f64, t12248: f64, t3427: f64, t20651: f64, t44017: f64, t6471: f64, t20644: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68754 = 0.57895126195293126241e3_f64 * t12227 * t6474 * t3385;
    let t68757 = 4.0_f64 * t3384 * t1733 * t16942;
    let t68760 = 0.96491876992155210402e2_f64 * t12248 * t6474 * t3427;
    let t68763 = 0.62071215503128080361e4_f64 * t44017 * t20651 * t3385;
    let t68766 = 2.0_f64 * t3384 * t6471 * t3427;
    let t68769 = 0.96491876992155210402e2_f64 * t12248 * t20644 * t3385;
    (t68754, t68757, t68760, t68763, t68766, t68769)
}
