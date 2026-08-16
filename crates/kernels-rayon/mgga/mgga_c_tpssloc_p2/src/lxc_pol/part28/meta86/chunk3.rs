//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 539/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk539(t25: f64, t28: f64, t1268: f64, t1442: f64, t1458: f64, t1408: f64, t514: f64, t1649: f64, t517: f64, t157: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1778 = 2.0_f64 * t1268 * t1458 + t1442;
    let t1782 = piecewise3(t26, 0.0_f64, 4.0_f64 / 3.0_f64 * t514 * t1408);
    let t1785 = piecewise3(t29, 0.0_f64, 4.0_f64 / 3.0_f64 * t517 * t1649);
    let t1787 = (t1782 + t1785) * t157;
    (t1778, t1787)
}
