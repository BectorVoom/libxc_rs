//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 902/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk902(t7230: f64, t7467: f64, t16524: f64, t8657: f64, t33185: f64, t1873: f64, t7801: f64, t3941: f64, t2039: f64, t12571: f64, t8662: f64, t7973: f64, t8301: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33645 = 0.135e2_f64 * t7230 * t7467;
    let t33653 = 27.0_f64 * t16524 * t8657;
    let t33655 = 27.0_f64 * t33185 * t8657;
    let t33656 = t7801 * t1873;
    let t33658 = 27.0_f64 * t3941 * t33656;
    let t33659 = t2039 * t7467;
    let t33661 = 27.0_f64 * t3941 * t33659;
    let t33669 = t12571 * t8662;
    let t33676 = t8301 * t7973;
    (t33645, t33653, t33655, t33656, t33658, t33659, t33661, t33669, t33676)
}
