//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1110/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1110(t570: f64, t6175: f64, t5636: f64, t1745: f64, t2009: f64, t1988: f64, t9549: f64, t1426: f64, t1579: f64, t2297: f64, t598: f64, t535: f64, t8539: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39169 = t570 * t6175;
    let t39171 = t570 * t5636;
    let t39173 = t2009 * t1745;
    let t39176 = t1988 * t9549;
    let t39182 = t598 * t1426 * t1579 * t2297;
    let t39186 = t598 * t1426 * t535 * t8539;
    (t39169, t39171, t39173, t39176, t39182, t39186)
}
