//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1185/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1185(t31747: f64, t652: f64, t7000: f64, t8607: f64, t7057: f64, t8526: f64, t532: f64, t8639: f64, t6879: f64, t1983: f64, t2314: f64, t8533: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31749 = 2.0_f64 * t652 * t31747;
    let t31750 = t8607 * t7000;
    let t31753 = 2.0_f64 * t8526 * t7057;
    let t31758 = t532 * t8639;
    let t31759 = t31758 * t6879;
    let t31761 = 3.0_f64 * t1983 * t31759;
    let t31769 = 2.0_f64 * t2314 * t8533;
    (t31749, t31750, t31753, t31758, t31759, t31761, t31769)
}
