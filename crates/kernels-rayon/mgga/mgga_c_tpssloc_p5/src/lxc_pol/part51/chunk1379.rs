//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1379/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1379(t114866: f64, t6552: f64, t7479: f64, t25341: f64, t31366: f64, t1880: f64, t26679: f64, t6553: f64, t6571: f64, t114592: f64, t118476: f64, t118479: f64, t118481: f64, t118484: f64, t121296: f64, t121299: f64, t121302: f64, t121305: f64, t121308: f64) -> f64 {
    let t121311 = t6552 * t114866 * t7479;
    let t121314 = t6552 * t31366 * t25341;
    let t121318 = t1880 * t6553 * t6571 * t26679;
    let t121320 = t118476 + t118479 - 0.82246703342411321824e-2_f64 * t114592 - t118481 + 0.19190897446562641759e-1_f64 * t121296 + 0.16449340668482264365e-1_f64 * t121299 + t118484 - 0.82246703342411321825e-2_f64 * t121302 + 0.41123351671205660912e-2_f64 * t121305 - 0.82246703342411321825e-2_f64 * t121308 - 0.16449340668482264365e-1_f64 * t121311 - 0.16449340668482264365e-1_f64 * t121314 - 0.82246703342411321825e-2_f64 * t121318;
    t121320
}
