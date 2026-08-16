//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1958/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1958(t2139: f64, t5022: f64, t471: f64, t1714: f64, t52: f64, t2132: f64, t24746: f64, t4997: f64, t7339: f64, t5001: f64, t7338: f64, t1730: f64, t7344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27603 = t2139 * t5022;
    let t27604 = t471 * t27603;
    let t27607 = t52 * t1714;
    let t27608 = t2132 * t27607;
    let t27609 = t27608 * t24746;
    let t27611 = t7339 * t4997;
    let t27614 = t5001 * t7338;
    let t27617 = t1730 * t7344;
    (t27603, t27604, t27607, t27608, t27609, t27611, t27614, t27617)
}
