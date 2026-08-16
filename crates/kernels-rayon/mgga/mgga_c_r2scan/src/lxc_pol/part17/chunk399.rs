//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 399/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk399(t1754: f64, t735: f64, t584: f64, t611: f64, t591: f64, t616: f64, t615: f64, t61: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1756 = 0.10843581300301739842e-1_f64 * t735 * t1754;
    let t1757 = t584 * t611;
    let t1758 = t616 * t591;
    let t1759 = t615 * t1758;
    let t1761 = 0.33872559466666666666e-2_f64 * t1757 * t1759;
    let t1762 = t61 * t625;
    (t1756, t1757, t1758, t1759, t1761, t1762)
}
