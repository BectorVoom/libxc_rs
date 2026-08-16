//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1071/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1071(t37833: f64, t2158: f64, t37699: f64, t10844: f64, t10899: f64, t2201: f64, t10848: f64, t2207: f64, t10894: f64, t1628: f64, t261: f64, t3299: f64, t6507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37834 = 0.89443204944342177673e-3_f64 * t37833;
    let t37835 = t37699 * t2158;
    let t37838 = t2201 * t10899 * t10844;
    let t37841 = t2207 * t10899 * t10848;
    let t37843 = t10894 * t1628;
    let t37848 = t3299 * t261 * t6507;
    (t37834, t37835, t37838, t37841, t37843, t37848)
}
