//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1456/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1456(t1983: f64, t33335: f64, t6999: f64, t8606: f64, t8944: f64, t26164: f64, t33211: f64, t7057: f64, t649: f64, t7467: f64, t2040: f64, t33363: f64, t7000: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122645 = t1983 * t33335 * t6999;
    let t122654 = t8606 * t8944;
    let t122656 = 2.0_f64 * t122654 * t26164;
    let t122659 = 2.0_f64 * t33211 * t7057;
    let t122660 = t649 * t7467;
    let t122662 = 2.0_f64 * t122660 * t2040;
    let t122664 = t33363 * t7000;
    (t122645, t122656, t122659, t122660, t122662, t122664)
}
