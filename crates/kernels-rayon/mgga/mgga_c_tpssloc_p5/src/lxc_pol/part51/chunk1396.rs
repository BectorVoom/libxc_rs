//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1396/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1396(t113005: f64, t114670: f64, t114673: f64, t114680: f64, t114689: f64, t114691: f64, t114694: f64, t118739: f64, t118743: f64, t118745: f64, t118751: f64, t121533: f64, t121536: f64, t121541: f64, t121546: f64, t121550: f64) -> f64 {
    let t121552 = 0.38381794893125283518e-1_f64 * t121533 - t118739 + t118743 - 0.19190897446562641759e-1_f64 * t114670 + t114673 + 0.19190897446562641759e-1_f64 * t121536 + t118745 + 0.41123351671205660912e-2_f64 * t114680 - t113005 - t114689 - 0.41123351671205660912e-2_f64 * t114691 + t114694 + 0.16449340668482264365e-1_f64 * t121541 - 0.82246703342411321825e-2_f64 * t121546 - t118751 + 0.82246703342411321825e-2_f64 * t121550;
    t121552
}
