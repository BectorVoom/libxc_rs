//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1176/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1176(t22633: f64, t31551: f64, t2015: f64, t7213: f64, t3887: f64, t2091: f64, t3886: f64) -> (f64, f64, f64) {
    let t31552 = t22633 * t31551;
    let t31554 = t7213 * t2015;
    let t31555 = t3887 * t31554;
    let t31558 = t3886 * t2091;
    (t31552, t31555, t31558)
}
