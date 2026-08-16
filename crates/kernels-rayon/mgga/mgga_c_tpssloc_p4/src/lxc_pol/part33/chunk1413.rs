//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1413/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1413(t55388: f64, t7769: f64, t28893: f64, t7467: f64, t100911: f64, t107545: f64, t107552: f64, t107555: f64, t107558: f64, t107566: f64, t107568: f64, t107570: f64, t107573: f64, t107575: f64, t107577: f64, t107579: f64, t107581: f64, t1458: f64, t2022: f64, t20347: f64, t22445: f64, t22448: f64, t23880: f64, t26523: f64, t5456: f64, t5493: f64, t577: f64, t7010: f64, t86647: f64) -> f64 {
    let t107583 = 81.0_f64 * t55388 * t7769;
    let t107585 = 81.0_f64 * t28893 * t7467;
    let t107588 = 27.0_f64 * t2022 * t22445 + 0.45e1_f64 * t107545 * t577 + 81.0_f64 * t23880 * t22448 + t107552 + t107555 + t107558 + 81.0_f64 * t86647 * t5456 + 0.405e2_f64 * t26523 * t5493 + 0.135e2_f64 * t7010 * t20347 + t107566 + t107568 + t107570 + t107573 + t107575 + t107577 + t107579 + t107581 + t107583 + t107585 + 0.405e2_f64 * t100911 * t1458;
    t107588
}
