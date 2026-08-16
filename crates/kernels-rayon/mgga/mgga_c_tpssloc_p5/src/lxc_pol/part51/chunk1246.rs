//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1246/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1246(t33185: f64, t8657: f64, t1873: f64, t7801: f64, t3941: f64, t2039: f64, t7467: f64, t1458: f64, t23880: f64, t26523: f64, t31795: f64, t33192: f64, t33195: f64, t33627: f64, t33641: f64, t33643: f64, t33645: f64, t33653: f64, t577: f64, t7010: f64, t7956: f64, t8508: f64) -> (f64, f64, f64) {
    let t33655 = 27.0_f64 * t33185 * t8657;
    let t33656 = t7801 * t1873;
    let t33658 = 27.0_f64 * t3941 * t33656;
    let t33659 = t2039 * t7467;
    let t33661 = 27.0_f64 * t3941 * t33659;
    let t33662 = 0.45e1_f64 * t33627 * t577 + 0.135e2_f64 * t31795 * t1458 + t33641 + t33643 + t33645 + 0.135e2_f64 * t26523 * t2039 + 27.0_f64 * t23880 * t7956 + 0.135e2_f64 * t7010 * t7801 + t33653 + t33655 + t33658 + t33661 + t33192 + t33195 + t8508;
    (t33656, t33659, t33662)
}
