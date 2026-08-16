//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1270/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1270(t25266: f64, t6019: f64, t6024: f64, t93054: f64, t18622: f64, t25245: f64, t5989: f64, t92978: f64, t25277: f64, t5985: f64, t18352: f64, t1945: f64, t807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106063 = t25266 * t6019;
    let t106065 = t93054 * t6024;
    let t106080 = t25245 * t18622;
    let t106082 = t92978 * t5989;
    let t106090 = t25277 * t5985;
    let t106102 = t807 * t1945 * t18352;
    (t106063, t106065, t106080, t106082, t106090, t106102)
}
