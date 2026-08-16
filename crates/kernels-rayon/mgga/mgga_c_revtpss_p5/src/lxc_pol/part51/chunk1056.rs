//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1056/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1056(t25698: f64, t7165: f64, t247: f64, t385: f64, t42792: f64, t8502: f64, t25610: f64, t93982: f64, t1078: f64, t11239: f64, t1982: f64, t8507: f64) -> (f64, f64, f64, f64) {
    let t120602 = t25698 * t7165;
    let t120624 = 0.62743259463289926663e-4_f64 * t8502 * t247 * t42792 * t385;
    let t120625 = t25610 * t93982;
    let t120636 = t1982 * t8507 * t11239 * t1078;
    (t120602, t120624, t120625, t120636)
}
