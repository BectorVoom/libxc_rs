//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2189/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2189(t1808: f64, t254: f64, t1377: f64, t6347: f64, t1385: f64, t22633: f64, t22635: f64, t1842: f64, t90516: f64, t1992: f64, t26355: f64, t90566: f64) -> (f64, f64, f64, f64) {
    let t97626 = t1808 * t254;
    let t97637 = t1377 * t6347;
    let t97640 = t22633 * t22635 * t97637 * t1385;
    let t97644 = t22633 * t22635 * t90516 * t1842;
    let t97647 = t1992 * t90566 * t26355;
    (t97626, t97640, t97644, t97647)
}
