//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1390/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1390(t3242: f64, t39103: f64, t136: f64, t3297: f64, t43713: f64, t43717: f64, t43721: f64, t43725: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43748: f64, t43750: f64) -> (f64, f64, f64) {
    let t43752 = t3242 * t39103;
    let t43754 = t136 * t3297 * t43752;
    let t43756 = -0.99342e0_f64 * t43713 - 0.11038e0_f64 * t43717 + 0.298026e1_f64 * t43721 + 0.66228e0_f64 * t43725 + 0.80513333333333333333e0_f64 * t43727 - 0.24154e1_f64 * t43729 + 0.20128333333333333334e1_f64 * t43734 - 0.72462e1_f64 * t43737 - 0.80513333333333333332e0_f64 * t43740 + 0.108693e2_f64 * t43743 + 0.24154e1_f64 * t43746 - 0.53675555555555555556e0_f64 * t43748 - 0.44729629629629629629e0_f64 * t43750 - 0.82785e-1_f64 * t43754;
    (t43752, t43754, t43756)
}
