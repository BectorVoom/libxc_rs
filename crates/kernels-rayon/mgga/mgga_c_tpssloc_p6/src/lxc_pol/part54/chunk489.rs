//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 489/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk489(t154: f64, t2559: f64, t222: f64, t2563: f64, t805: f64, t68: f64, t808: f64) -> (f64, f64, f64, f64) {
    let t2600 = t2559 * t154;
    let t2602 = 35.0_f64 / 432.0_f64 * t2600 * t222;
    let t2603 = t2563 * t805;
    let t2617 = t808 * t68;
    (t2600, t2602, t2603, t2617)
}
