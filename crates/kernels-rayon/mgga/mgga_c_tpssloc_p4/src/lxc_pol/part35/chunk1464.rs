//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1464/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1464(t108888: f64, t108918: f64, t109966: f64, t109980: f64, t105105: f64, t107552: f64, t107555: f64, t107558: f64, t107566: f64, t107568: f64, t107570: f64, t107573: f64, t107575: f64, t107577: f64, t107579: f64, t107581: f64, t107583: f64, t107585: f64, t1458: f64, t20347: f64, t2169: f64, t22445: f64, t22448: f64, t24972: f64, t27921: f64, t5456: f64, t5493: f64, t577: f64, t7423: f64, t96334: f64) -> (f64, f64) {
    let t109982 = t108888 + t108918 + t109966 + t109980;
    let t110002 = t107552 + 0.45e1_f64 * t109982 * t577 + t107555 + t107558 + 0.135e2_f64 * t7423 * t20347 + t107566 + t107568 + t107570 + 81.0_f64 * t24972 * t22448 + 0.405e2_f64 * t105105 * t1458 + 0.405e2_f64 * t27921 * t5493 + t107573 + t107575 + t107577 + t107579 + t107581 + 81.0_f64 * t96334 * t5456 + t107583 + 27.0_f64 * t2169 * t22445 + t107585;
    (t109982, t110002)
}
