//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 754/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk754(t4997: f64, t7339: f64, t5001: f64, t7338: f64, t1730: f64, t7344: f64, t4993: f64, t7345: f64, t5040: f64, t7310: f64, t27607: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27611 = t7339 * t4997;
    let t27614 = t5001 * t7338;
    let t27617 = t1730 * t7344;
    let t27622 = t7345 * t4993;
    let t27626 = t7310 * t5040;
    let t27628 = t27607 * t460;
    (t27611, t27614, t27617, t27622, t27626, t27628)
}
