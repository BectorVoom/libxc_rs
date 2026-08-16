//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1246/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1246(t34104: f64, t34115: f64, t34157: f64, t34173: f64, t3: f64, t1458: f64, t2039: f64, t24972: f64, t27921: f64, t32406: f64, t33192: f64, t33195: f64, t33641: f64, t33643: f64, t33645: f64, t33653: f64, t33655: f64, t33658: f64, t33661: f64, t577: f64, t7423: f64, t7801: f64, t7956: f64, t8508: f64) -> (f64, f64, f64) {
    let t34175 = t34104 + t34115 + t34157 + t34173;
    let t34176 = t3 * t34175;
    let t34194 = 0.45e1_f64 * t34175 * t577 + 0.135e2_f64 * t32406 * t1458 + 0.135e2_f64 * t27921 * t2039 + 27.0_f64 * t24972 * t7956 + 0.135e2_f64 * t7423 * t7801 + t33641 + t33643 + t33645 + t33653 + t33655 + t33658 + t33661 + t33192 + t33195 + t8508;
    (t34175, t34176, t34194)
}
