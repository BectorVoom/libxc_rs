//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1016/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1016(t5758: f64, t932: f64, t2888: f64, t5742: f64, t2892: f64, t4335: f64, t5679: f64, t5683: f64, t5687: f64, t324: f64, t1580: f64) -> (f64, f64, f64, f64, f64) {
    let t5759 = t5758 * t932;
    let t5762 = t5742 * t2888;
    let t5769 = t2892 + 0.61805555555555555556e-2_f64 * t4335 - 0.61805555555555555555e-2_f64 * t5679 + 0.18541666666666666667e-1_f64 * t5683 - 0.92708333333333333333e-2_f64 * t5687;
    let t5770 = t5769 * t324;
    let t5774 = t1580 * t1580;
    (t5759, t5762, t5769, t5770, t5774)
}
