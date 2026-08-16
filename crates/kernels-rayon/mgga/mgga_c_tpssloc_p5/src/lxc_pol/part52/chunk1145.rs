//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1145/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1145(t8362: f64, t865: f64, t2718: f64, t8352: f64, t10110: f64, t6547: f64, t8332: f64, t6571: f64, t6662: f64, t6553: f64, t1880: f64, t23204: f64, t8335: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30646 = t8362 * t865;
    let t30647 = t2718 * t30646;
    let t30650 = t8352 * t865;
    let t30651 = t10110 * t30650;
    let t30655 = 0.38381794893125283518e-1_f64 * t6547 * t8332;
    let t30656 = t6571 * t6662;
    let t30657 = t6553 * t30656;
    let t30659 = 0.16449340668482264365e-1_f64 * t1880 * t30657;
    let t30660 = t23204 * t8335;
    (t30647, t30651, t30655, t30656, t30657, t30659, t30660)
}
