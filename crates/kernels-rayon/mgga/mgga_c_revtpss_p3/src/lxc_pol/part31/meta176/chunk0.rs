//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 851/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk851(t4669: f64, t954: f64, t1621: f64, t2970: f64, t953: f64, t2848: f64, t2974: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t324: f64) -> (f64, f64, f64, f64, f64) {
    let t4670 = t4669 * t954;
    let t4673 = t1621 * t2970;
    let t4674 = t4673 * t953;
    let t4682 = t2974 + 0.30902777777777777778e-2_f64 * t2848 + 0.30902777777777777778e-2_f64 * t4571 - 0.61805555555555555555e-2_f64 * t4576 + 0.18541666666666666667e-1_f64 * t4581 - 0.92708333333333333333e-2_f64 * t4585;
    let t4683 = t4682 * t324;
    (t4670, t4673, t4674, t4682, t4683)
}
