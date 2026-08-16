//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1236/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1236(t2039: f64, t33211: f64, t7467: f64, t88: f64, t7801: f64, t8601: f64, t31758: f64, t7687: f64, t1983: f64, t5161: f64, t8640: f64, t7688: f64, t8607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33595 = 2.0_f64 * t33211 * t2039;
    let t33596 = t88 * t7467;
    let t33598 = 2.0_f64 * t33596 * t2039;
    let t33600 = 2.0_f64 * t8601 * t7801;
    let t33603 = t31758 * t7687;
    let t33605 = 3.0_f64 * t1983 * t33603;
    let t33610 = t8640 * t5161;
    let t33611 = t1983 * t33610;
    let t33615 = 3.0_f64 * t8607 * t7688;
    (t33595, t33596, t33598, t33600, t33603, t33605, t33610, t33611, t33615)
}
