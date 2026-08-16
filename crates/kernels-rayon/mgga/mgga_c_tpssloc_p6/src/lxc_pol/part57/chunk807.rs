//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 807/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk807(t1983: f64, t28827: f64, t1799: f64, t1845: f64, t8643: f64, t22574: f64, t1390: f64, t6347: f64, t6878: f64, t7685: f64, t7688: f64, t7754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28829 = 6.0_f64 * t1983 * t28827;
    let t28830 = t1799 * t1845;
    let t28831 = t8643 * t28830;
    let t28833 = 6.0_f64 * t22574 * t28831;
    let t28834 = t1390 * t6347;
    let t28835 = t6878 * t28834;
    let t28837 = 3.0_f64 * t1983 * t28835;
    let t28841 = 6.0_f64 * t7685 * t7688;
    let t28843 = 2.0_f64 * t7685 * t7754;
    (t28829, t28830, t28831, t28833, t28834, t28835, t28837, t28841, t28843)
}
