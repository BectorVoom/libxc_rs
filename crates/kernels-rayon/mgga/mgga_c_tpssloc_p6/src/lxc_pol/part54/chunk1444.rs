//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1444/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1444(t27226: f64, t8526: f64, t1983: f64, t33335: f64, t6999: f64, t8606: f64, t8944: f64, t26164: f64, t33211: f64, t7057: f64, t649: f64, t7467: f64) -> (f64, f64, f64, f64, f64) {
    let t122627 = 2.0_f64 * t8526 * t27226;
    let t122645 = t1983 * t33335 * t6999;
    let t122654 = t8606 * t8944;
    let t122656 = 2.0_f64 * t122654 * t26164;
    let t122659 = 2.0_f64 * t33211 * t7057;
    let t122660 = t649 * t7467;
    (t122627, t122645, t122656, t122659, t122660)
}
