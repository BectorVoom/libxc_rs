//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 422/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk422(t12: f64, t20: f64, t1392: f64, t1395: f64, t640: f64, t1399: f64, t1732: f64, t1734: f64) -> (f64, f64, f64, f64, f64) {
    let t1736 = 1.0_f64/pow_3_2(t12);
    let t1737 = t1736 * t20;
    let t1738 = t1737 * t1392;
    let t1740 = t640 * t1395;
    let t1743 = 0.17261666666666666666e1_f64 * t1732 - 0.46031111111111111111e1_f64 * t1734 - 0.73354999999999999999e-1_f64 * t1738 + 0.14671e0_f64 * t1740 + 0.11038e0_f64 * t1399;
    (t1736, t1737, t1738, t1740, t1743)
}
