//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 887/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk887(t22633: f64, t31551: f64, t2015: f64, t7213: f64, t3887: f64, t2091: f64, t3886: f64, t1385: f64, t22635: f64, t1992: f64, t8636: f64, t794: f64, t8611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31552 = t22633 * t31551;
    let t31554 = t7213 * t2015;
    let t31555 = t3887 * t31554;
    let t31558 = t3886 * t2091;
    let t31559 = t31558 * t1385;
    let t31560 = t22635 * t31559;
    let t31561 = t1992 * t31560;
    let t31563 = t8636 * t1385;
    let t31564 = t3887 * t31563;
    let t31569 = t794 * t8611;
    (t31552, t31555, t31558, t31559, t31560, t31561, t31564, t31569)
}
