//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1165/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1165(t552: f64, t6604: f64, t1338: f64, t7722: f64, t7696: f64, t794: f64, t6897: f64, t225: f64, t7704: f64, t25049: f64, t25277: f64, t25077: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26446 = t6604 * t552;
    let t26458 = t1338 * t7722;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26477 = t7704 * t225;
    let t26591 = 0.38381794893125283518e-1_f64 * t25049;
    let t26613 = 0.38381794893125283518e-1_f64 * t25277;
    let t26619 = 7.0_f64 / 288.0_f64 * t25077;
    (t26446, t26458, t26474, t26475, t26477, t26591, t26613, t26619)
}
