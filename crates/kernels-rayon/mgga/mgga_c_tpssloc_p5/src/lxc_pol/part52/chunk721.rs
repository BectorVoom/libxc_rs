//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 721/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk721(t815: f64, t829: f64, t6605: f64, t1898: f64, t808: f64, t249: f64, t59: f64, t814: f64) -> (f64, f64, f64, f64, f64) {
    let t6606 = t815 * t829;
    let t6607 = t6605 * t6606;
    let t6609 = t808 * t1898;
    let t6610 = t6609 * t249;
    let t6612 = t814 * t59;
    (t6606, t6607, t6609, t6610, t6612)
}
