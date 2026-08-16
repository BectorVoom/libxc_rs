//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1224/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1224(t5107: f64, t652: f64, t8326: f64, t32783: f64, t6876: f64, t1845: f64, t6995: f64, t26161: f64, t26162: f64, t31537: f64, t7468: f64, t31540: f64) -> (f64, f64, f64, f64, f64) {
    let t119830 = 2.0_f64 * t652 * t5107 * t8326;
    let t119831 = t6876 * t32783;
    let t119832 = t1845 * t6995;
    let t119835 = 4.0_f64 * t26161 * t26162 * t119832;
    let t119837 = 4.0_f64 * t31537 * t7468;
    let t119839 = 4.0_f64 * t31540 * t7468;
    (t119830, t119831, t119835, t119837, t119839)
}
