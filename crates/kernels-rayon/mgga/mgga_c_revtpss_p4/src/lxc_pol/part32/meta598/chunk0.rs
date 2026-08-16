//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1932/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1932(t2110: f64, t5808: f64, t1455: f64, t8130: f64, t1921: f64, t7541: f64, t28944: f64, t575: f64, t5891: f64, t94978: f64, t665: f64, t94982: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104079 = 2.0_f64 * t2110 * t5808;
    let t104081 = 2.0_f64 * t1455 * t8130;
    let t104083 = 2.0_f64 * t7541 * t1921;
    let t104085 = 2.0_f64 * t28944 * t575;
    let t105870 = t94978 * t5891;
    let t105872 = t5891 * t665;
    let t105873 = t94982 * t105872;
    (t104079, t104081, t104083, t104085, t105870, t105873)
}
